// https://github.com/clap-rs/clap/blob/master/examples/git-derive.rs
// https://docs.rs/clap/latest/clap/_derive/index.html#terminology
use anyhow::{bail, Result};
use clap::{Parser, Subcommand};
use npmpink_core::package::Package;
use npmpink_core::source::Source;
use npmpink_core::target::Target;
use npmpink_core::utils::packages_from_source;
use npmpink_core::{
    config::{appConfig, Config},
    workspace::Workspace,
};
use std::path::PathBuf;

use crate::prompts::select_packages;

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
    /// Interactive manage the packages in current workspace.
    Change,
}

impl PackageSubCli {
    /// reduce from sources
    pub(super) fn workspaces(&self) -> Vec<Workspace> {
        let config = appConfig.lock().unwrap();

        // for source in config.sources.iter() {
        //     println!("{}: {}", source.id, source.path.display());
        // }

        config
            .sources
            .iter()
            .map(|s| Workspace::init_from_dir(&s.path))
            .collect()
    }
}

pub(super) fn run() -> Result<()> {
    let mut cli = Cli::parse();
    if cli.cwd.is_none() {
        cli.cwd = std::env::current_dir().ok();
    }

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
            return Ok(());
        }
        None => {}
    }

    Ok(())
}

struct InitArgs {
    force: bool,
}
fn cmd_handler_init(args: &InitArgs) -> Result<()> {
    if !args.force && Config::healthcheck().is_ok() {
        println!("init passed");
        return Ok(());
    }

    // init config
    Config::create_from_default()
}

fn cmd_handler_check() -> Result<()> {
    let result = Config::healthcheck();

    if result.is_ok() {
        println!("all checks pass");
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

    let Some(absolute_dir) = wk.absolute_dir() else {
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
    let Some(absolute_dir) = wk.absolute_dir() else {
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
        println!("{}: {}", source.id, source.path.display());
    }

    Ok(())
}

/// handle packages, like list packages from all sources.
fn cmd_handler_package_sub_cli(cli: &Cli, command: &PackageSubCli) -> Result<()> {
    // TODO: check current lockfile in the current workspace.
    match command {
        PackageSubCli::Add => {}
        PackageSubCli::Remove => {}
        PackageSubCli::Change => {
            cmd_handler_package_change(cli, command)?;
        }
    }
    Ok(())
}

/// List packages from sources that not in the lockfile.
/// If multiple packages with same name is selected, override the old one in lockfile.
fn cmd_handler_package_add(cli: &Cli) -> Result<()> {
    Ok(())
}

/// https://github.com/mikaelmello/inquire/blob/main/inquire/examples/multiselect.rs
/// Change the workspace's packages.
fn cmd_handler_package_change(cli: &Cli, package_cmd: &PackageSubCli) -> Result<()> {
    let config = appConfig.lock().unwrap();
    let pkgs = config
        .sources
        .iter()
        .flat_map(packages_from_source)
        .collect::<Vec<Package>>();

    let picked = select_packages(&pkgs)?;

    let target = Target::init_from_dir(cli.cwd.as_ref().unwrap());
    {
        let mut lockfile = target.lockfile_mut()?;

        for pkg in picked.iter().cloned().cloned() {
            lockfile.add_package(pkg.name.clone(), pkg);
        }
    }
    target.flush_lockfile()?;

    /////////
    println!("done");

    // ws.flush_lockfile()?;
    Ok(())
}
