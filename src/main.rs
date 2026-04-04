mod cli;
mod config;
mod scaffold;
mod template_engine;
mod wizard;

use anyhow::Result;
use clap::Parser;
use cli::Args;

fn main() -> Result<()> {
    let args = Args::parse();
    let config = wizard::resolve(args)?;
    scaffold::run(&config)?;
    Ok(())
}
