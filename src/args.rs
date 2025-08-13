//! cli args + parser

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about=None)]
pub(crate) struct Args {
    #[arg(short, long)]
    pub project_path: PathBuf,

    pub max_depth: Option<u32>,

    #[command(subcommand)]
    pub level: Level,
}

#[derive(Subcommand)]
pub enum Level {
    Crate,
    Module {
        #[arg(long)]
        crate_name: String,
    },
}
