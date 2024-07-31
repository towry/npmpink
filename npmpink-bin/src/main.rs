mod cli;
mod config;

use cli::*;
use npmpink_tui::shell::shell;

fn main() {
    let result = run();

    if let Err(e) = result {
        handle_error(e);
    }
}

// FIXME: how to print out traceback errors?
fn handle_error(e: anyhow::Error) {
    if let Some(clap_err) = e.downcast_ref::<clap::Error>() {
        let exit_code = if clap_err.use_stderr() { 1 } else { 0 };
        let _ = clap_err.print();
        std::process::exit(exit_code);
    }

    if let Ok(ref mut sh) = shell() {
        let _ = sh.error(&format!("{}", e));
    }
    std::process::exit(1);
}
