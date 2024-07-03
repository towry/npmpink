use crate::config::Config;
use clap::{Parser, Subcommand};
use std::{io::Error, result::Result};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub(super) enum Commands {
    /// Setup if is first run, create root config file etc.
    Init,

    /// Source manage.
    Source(SourceSubCli),

    /// Check packages in current workspace.
    Check,
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
    Add,
    /// Remove source.
    Remove,
    /// List source.
    List,
}

pub(super) fn run() -> Result<(), Error> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init) => {
            return cmd_handler_init();
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

fn cmd_handler_init() -> Result<(), Error> {
    if Config::healthcheck().is_ok() {
        println!("init passed");
        return Ok(());
    }

    // init config
    Config::init_from_default()
}

fn cmd_handler_check() -> Result<(), Error> {
    let result = Config::healthcheck();

    if result.is_ok() {
        println!("all checks pass");
        return Ok(());
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        result.unwrap_err().to_string(),
    ))
}

fn cmd_handler_source_sub_cli(command: &Option<SourceCommands>) -> Result<(), Error> {
    match command {
        Some(SourceCommands::Add) => {
            println!("add source..");
        }
        Some(SourceCommands::Remove) => {
            println!("remove source..");
        }
        Some(SourceCommands::List) => {
            println!("list sources..");
        }
        None => {}
    }
    Ok(())
}
