// https://github.com/clap-rs/clap/blob/master/examples/git-derive.rs
use crate::source::Source;
use crate::{
    config::{appConfig, Config},
    workspace::Workspace,
};
use clap::{Args, Parser, Subcommand};
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
    Init(InitArgs),

    /// Source manage.
    Source(SourceSubCli),

    /// Check packages in current workspace.
    Check,
}

#[derive(Debug, Args)]
struct InitArgs {
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
    Add {
        #[arg(short, long, help = "dir to add")]
        dir: String,
    },
    /// Remove source.
    Remove,
    /// List source.
    List,
}

pub(super) fn run() -> Result<(), Error> {
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

fn cmd_handler_init(args: &InitArgs) -> Result<(), Error> {
    if !args.force && Config::healthcheck().is_ok() {
        println!("init passed");
        return Ok(());
    }

    // init config
    Config::create_from_default()
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
        Some(SourceCommands::Add { dir }) => {
            cmd_handler_source_add(dir)?;
        }
        Some(SourceCommands::Remove) => {
            println!("remove source..");
        }
        Some(SourceCommands::List) => {
            cmd_handler_source_list()?;
        }
        None => {}
    }
    Ok(())
}

fn cmd_handler_source_add(dir: &String) -> Result<(), Error> {
    let wk = Workspace::init_from_dir(dir);

    if !wk.is_ok_loosely() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "workspace doesn't contains package.json",
        ));
    }

    let Ok(mut config) = appConfig.lock() else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to get app config",
        ));
    };

    let Some(absolute_dir) = wk.absolute_dir() else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Not an valid directory",
        ));
    };

    let source = Source::new(absolute_dir);

    if config.has_source(&source.id) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Source already exists",
        ));
    }

    config.sources.push(source);
    config.flush()?;

    Ok(())
}

fn cmd_handler_source_list() -> Result<(), Error> {
    let config = appConfig.lock().unwrap();

    for source in config.sources.iter() {
        println!("{}: {}", source.id, source.path.display());
    }

    Ok(())
}
