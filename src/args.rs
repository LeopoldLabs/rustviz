//! cli args + parser

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Clone, ValueEnum)]
pub enum Level {
    Crate,
    Module,
}

#[derive(Parser)]
pub(crate) struct Args {
    pub project_path: PathBuf,

    pub level: Level,
}
