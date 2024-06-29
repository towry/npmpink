mod cli;
mod config;
mod lockfile;
mod package;
mod source;
mod tui;

use cli::*;
use std::io;

fn main() -> Result<(), io::Error> {
    run()
}

// healthy report
fn check() {}
// toggle from the panel.
fn toggle_package() {}
fn add_source() {}
fn add_package() {}
fn remove_source() {}
fn remove_package() {}
// use EDITOR to open the config
// or print the location of the edit
fn edit_config() {}
