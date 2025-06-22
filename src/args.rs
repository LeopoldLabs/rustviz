//! cli args + parser

use std::path::{PathBuf};

use clap::Parser;

#[derive(Parser)]
pub(crate) struct Args {
    pub filepath: PathBuf,

    pub max_depth: Option<u32>,

    #[arg(long, action)]
    pub include_private: bool,
}