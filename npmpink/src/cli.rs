// https://github.com/clap-rs/clap/blob/master/examples/git-derive.rs
// https://docs.rs/clap/latest/clap/_derive/index.html#terminology
use crate::source::Source;
use crate::{
    config::{appConfig, Config},
    workspace::Workspace,
};
use anyhow::{bail, Result};
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub(super) enum Commands {
    /// Setup if is first run, create root config file etc.
    Init(InitArgs),

    /// Source manage.
    Source(SourceSubCli),

    /// Check packages in current workspace.
    Check,
}

#[derive(Debug, Args)]
pub(crate) struct InitArgs {
    #[arg(short, long, help = "Force init", action)]
    force: bool,
}

#[derive(Debug, Parser)]
#[command(arg_required_else_help = true)]
pub(super) struct SourceSubCli {
    #[command(subcommand)]
    command: Option<SourceCommands>,
}

#[derive(Debug, Subcommand)]
pub(super) enum SourceCommands {
    /// Add source.
    Add { dir: String },
    /// Remove source.
    Remove { dir: String },
    /// List source.
    List,
}

pub(super) fn run() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init(args)) => {
            return cmd_handler_init(args);
        }
        Some(Commands::Source(command)) => {
            return cmd_handler_source_sub_cli(&command.command);
        }
        Some(Commands::Check) => {
            return cmd_handler_check();
        }
        None => {}
    }

    Ok(())
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

fn cmd_handler_source_sub_cli(command: &Option<SourceCommands>) -> Result<()> {
    match command {
        Some(SourceCommands::Add { dir }) => {
            cmd_handler_source_add(dir)?;
        }
        Some(SourceCommands::Remove { dir }) => {
            cmd_handler_source_remove(dir)?;
        }
        Some(SourceCommands::List) => {
            cmd_handler_source_list()?;
        }
        None => {}
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
