#![feature(exit_status_error)]

use anyhow::Result;
use clap::Parser;

mod args;
mod c4;
mod rustdoc;
mod walker;

use args::Args;

fn main() -> Result<()> {
    let args = Args::try_parse()?;

    let crates = rustdoc::docs(&args.project_path)?;

    let diagram = walker::walker(args.max_depth, crates, args.level);

    println!("{diagram}");

    Ok(())
}
