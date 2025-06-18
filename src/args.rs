//! cli args + parser

use std::path::{PathBuf};

use clap::Parser;

#[derive(Parser)]
pub(crate) struct Args {
    pub filepath: PathBuf,

    #[arg(long, action)]
    pub include_private: bool,
}