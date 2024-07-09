// https://github.com/clap-rs/clap/blob/master/examples/git-derive.rs
// https://docs.rs/clap/latest/clap/_derive/index.html#terminology
use crate::source::Source;
use crate::{
    config::{appConfig, Config},
    workspace::Workspace,
};
use anyhow::{bail, Result};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub(super) enum Commands {
    /// Setup if is first run, create root config file etc.
    Init {
        #[arg(short, long, help = "Force init", action)]
        force: bool,
    },

    /// Source manage.
    Source {
        #[command(subcommand)]
        command: SourceSubCli,
    },

    /// Package manage.
    Package {
        #[command(subcommand)]
        command: PackageSubCli,
    },

    /// Check packages in current workspace.
    Check,
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

pub(super) fn run() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init { force }) => {
            return cmd_handler_init(&InitArgs { force: *force });
        }
        Some(Commands::Source { command }) => {
            return cmd_handler_source_sub_cli(command);
        }
        Some(Commands::Package { command }) => {
            return cmd_handler_package_sub_cli(command);
        }
        Some(Commands::Check) => {
            return cmd_handler_check();
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

    if !wk.is_ok_loosely() {
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

fn cmd_handler_package_sub_cli(command: &PackageSubCli) -> Result<()> {
    match command {
        PackageSubCli::Add => {}
        PackageSubCli::Remove => {}
        PackageSubCli::Change => {}
    }
    Ok(())
}
