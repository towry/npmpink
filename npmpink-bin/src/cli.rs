// https://github.com/clap-rs/clap/blob/master/examples/git-derive.rs
// https://docs.rs/clap/latest/clap/_derive/index.html#terminology
use crate::config::{appConfig, Config, HealthCheckError};
use anyhow::{bail, Result};
use clap::{Parser, Subcommand};
use npmpink_core::item_formatter::PackageItemFormatter;
use npmpink_core::ops::packages::{difference_packages, packages_from_source};
use npmpink_core::package::Package;
use npmpink_core::source::Source;
use npmpink_core::target::Target;
use npmpink_core::workspace::Workspace;
use npmpink_tui::item::PackageItemDisplay;
use npmpink_tui::select::pick_items;
use npmpink_tui::shell::shell;
use std::cell::{RefCell, RefMut};
use std::path::PathBuf;
use std::process::Command;
use std::rc::Rc;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(
        long,
        hide_default_value = true,
        help = "Current workspace dir to run cli"
    )]
    cwd: Option<PathBuf>,
    #[clap(skip)]
    target: Option<RefCell<Target>>,
}

impl Cli {
    pub(super) fn target(&self) -> RefMut<'_, Target> {
        self.target.as_ref().unwrap().borrow_mut()
    }
}

#[derive(Subcommand, Debug)]
pub(super) enum Commands {
    /// Setup if is first run, create root config file etc.
    Init {
        #[arg(short, long, help = "Force init", action)]
        force: bool,
    },

    /// Manage sources, source is target dir to look packages.
    Source {
        #[command(subcommand)]
        command: SourceSubCli,
    },

    /// Manage packages, add, remove package from sources in current workspace.
    Package {
        #[command(subcommand)]
        command: PackageSubCli,
    },

    /// Check packages in current workspace.
    Check,

    /// Sync added packages to node_modules
    Sync,
}

#[derive(Debug, Subcommand)]
pub(super) enum SourceSubCli {
    /// Add source.
    Add { dir: String },
    /// Remove source.
    Remove { dir: String },
    /// List source.
    List,
}

#[derive(Debug, Subcommand)]
#[command(arg_required_else_help = true)]
pub(super) enum PackageSubCli {
    /// Manually add package by name to current workspace.
    /// The package must be within the sources.
    Add,
    /// Manually remove previously added package from current workspace.
    /// The package must be within the sources.
    Remove,
    /// list all
    List,
}

pub(super) fn run() -> Result<()> {
    let mut cli = Cli::parse();
    if cli.cwd.is_none() {
        cli.cwd = std::env::current_dir().ok();
    }
    cli.target = Some(RefCell::new(Target::init_from_dir(
        cli.cwd.as_ref().unwrap(),
    )));

    match &cli.command {
        Some(Commands::Init { force }) => {
            return cmd_handler_init(&InitArgs { force: *force });
        }
        Some(Commands::Source { command }) => {
            return cmd_handler_source_sub_cli(command);
        }
        Some(Commands::Package { command }) => {
            return cmd_handler_package_sub_cli(&cli, command);
        }
        Some(Commands::Check) => {
            return cmd_handler_check();
        }
        Some(Commands::Sync) => {
            return cmd_handler_sync(&cli);
        }
        None => {}
    }

    Ok(())
}

struct InitArgs {
    force: bool,
}
fn cmd_handler_init(args: &InitArgs) -> Result<()> {
    let result = Config::healthcheck();
    match result {
        Err(err) => match err {
            HealthCheckError::ConfigFileNotExist => {}
            _ => {
                return Err(anyhow::Error::msg(err));
            }
        },
        Ok(_) => {
            if !args.force {
                shell()?.info("npmpink is already initialized")?;

                return Ok(());
            }
        }
    }

    // init config
    Config::create_from_default()?;

    shell()?.info("inited config file")
}

/// Update packages inside npmpink.lock to node modules
fn cmd_handler_sync(cli: &Cli) -> Result<()> {
    let target = cli.target();
    let lockfile_pkgs = {
        let lockfile = target.lockfile()?;
        lockfile.packages_iter().collect::<Vec<Package>>()
    };
    let pkgs_paths = lockfile_pkgs.iter();
    let mut sh = shell()?;

    for pkg in pkgs_paths {
        sh.info(format!("> Link package {}: \n", pkg.name))?;
        Command::new("pnpm")
            .args(["link", &pkg.dir])
            .status()
            .map(|_| ())
            .map_err(anyhow::Error::msg)?;
        sh.info("\n")?;
    }

    Ok(())
}

fn cmd_handler_check() -> Result<()> {
    let result = Config::healthcheck();

    if result.is_ok() {
        shell()?.info("all checks pass")?;
        return Ok(());
    }

    bail!("check pass failed")
}

fn cmd_handler_source_sub_cli(command: &SourceSubCli) -> Result<()> {
    match command {
        SourceSubCli::Add { dir } => {
            cmd_handler_source_add(dir)?;
        }
        SourceSubCli::Remove { dir } => {
            cmd_handler_source_remove(dir)?;
        }
        SourceSubCli::List => {
            cmd_handler_source_list()?;
        }
    }
    Ok(())
}

fn cmd_handler_source_add(dir: &String) -> Result<()> {
    let wk = Workspace::init_from_dir(dir);

    if !wk.has_package_json() {
        bail!("workspace doesn't contains package.json");
    }

    let Ok(mut config) = appConfig.lock() else {
        bail!("Failed to get app config");
    };

    let Some(absolute_dir) = wk.absolute_dir().ok() else {
        bail!("Not an valid directory");
    };

    let source = Source::new(absolute_dir);

    if config.has_source(&source.id) {
        bail!("Source already exists");
    }

    config.sources.push(source);
    config.flush()?;

    Ok(())
}

fn cmd_handler_source_remove(dir: &String) -> Result<()> {
    let wk = Workspace::init_from_dir(dir);

    let Ok(mut config) = appConfig.lock() else {
        bail!("Failed to get app config");
    };
    let Some(absolute_dir) = wk.absolute_dir().ok() else {
        bail!("Not an valid directory");
    };

    let source = Source::new(absolute_dir);
    if !config.has_source(&source.id) {
        return Ok(());
    }

    config.sources.retain(|s| s.id != source.id);
    config.flush()?;

    Ok(())
}

fn cmd_handler_source_list() -> Result<()> {
    let config = appConfig.lock().unwrap();

    for source in config.sources.iter() {
        shell()?.info(format!("{}: {}", source.id, source.path.display()))?;
    }

    Ok(())
}

/// handle packages, like list packages from all sources.
fn cmd_handler_package_sub_cli(cli: &Cli, command: &PackageSubCli) -> Result<()> {
    match command {
        PackageSubCli::Add => {
            cmd_handler_package_add(cli)?;
        }
        PackageSubCli::Remove => {
            cmd_handler_package_remove(cli)?;
        }
        PackageSubCli::List => {
            cmd_handler_package_list_all(cli)?;
        }
    }
    Ok(())
}

fn cmd_handler_package_list_all(_cli: &Cli) -> Result<()> {
    let config = appConfig.lock().unwrap();
    let mut pkgs = config.sources.iter().flat_map(packages_from_source);

    let mut sh = shell()?;

    if pkgs.by_ref().peekable().peek().is_none() {
        sh.warn("no packages to list")?;
    }

    for pkg in pkgs {
        sh.info(format!("{:?}", pkg.name))?;
    }

    Ok(())
}

/// https://github.com/mikaelmello/inquire/blob/main/inquire/examples/multiselect.rs
/// Change the workspace's packages.
fn cmd_handler_package_add(cli: &Cli) -> Result<()> {
    let config = appConfig.lock().unwrap();
    let target = cli.target();
    let pkgs = config
        .sources
        .iter()
        .flat_map(packages_from_source)
        .collect::<Vec<Package>>();
    let lockfile_pkgs = {
        let lockfile = target.lockfile()?;
        lockfile.packages_iter().collect::<Vec<Package>>()
    };

    let get_weak_source =
        |source_id: &String| config.sources.iter().find(|s| &s.id == source_id).unwrap();

    let pkgs_to_pick = difference_packages(&pkgs, &lockfile_pkgs)
        .into_iter()
        .map(Rc::new);

    let picked = pick_items(
        pkgs_to_pick
            .map(|p| {
                let weak_source = get_weak_source(&p.source_id);
                PackageItemDisplay::new(PackageItemFormatter::new(Rc::clone(&p), weak_source))
            })
            .collect::<Vec<PackageItemDisplay>>()
            .as_slice(),
        Some(Default::default()),
    )?;

    {
        let mut lockfile = target.lockfile_mut()?;

        for pkg in picked.iter().cloned() {
            let raw = pkg.raw;
            lockfile.add_package(raw.inner.name.clone(), Rc::unwrap_or_clone(raw.inner));
        }
    }
    target.flush_lockfile()?;

    shell()?.info(format!("{} packages added", picked.len()))?;
    Ok(())
}

// TODO: package and source existence check
fn cmd_handler_package_remove(cli: &Cli) -> Result<()> {
    let config = appConfig.lock().unwrap();
    let target = cli.target();

    let lockfile_pkgs = {
        let lockfile = target.lockfile()?;
        lockfile.packages_iter().collect::<Vec<Package>>()
    };

    let get_weak_source =
        |source_id: &String| config.sources.iter().find(|s| &s.id == source_id).unwrap();

    let pkgs_to_pick = lockfile_pkgs
        .into_iter()
        .map(Rc::new)
        .map(|p| {
            PackageItemDisplay::new(PackageItemFormatter::new(
                Rc::clone(&p),
                get_weak_source(&p.source_id),
            ))
        })
        .collect::<Vec<PackageItemDisplay>>();

    let picked = pick_items(pkgs_to_pick.as_slice(), Some(Default::default()))?;
    {
        let mut lockfile = target.lockfile_mut()?;

        for pkg in picked.iter().cloned() {
            lockfile.remove_package(pkg.raw.inner.name.clone());
        }
    }
    target.flush_lockfile()?;

    shell()?.info(format!("{} packages removed", picked.len()))?;
    Ok(())
}
