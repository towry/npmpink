mod cli;
mod select_packages;

use cli::*;

fn main() -> anyhow::Result<()> {
    run()
}
