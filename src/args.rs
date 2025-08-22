//! cli args + parser

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about=None)]
pub struct Args {
    #[arg(default_value = ".")]
    pub project_path: PathBuf,

    #[arg(short, long, default_value = "-")]
    pub output_path: PathBuf,

    #[arg(short = 'f', long, default_value = "graphviz")]
    pub output_format: OutputFormat,

    #[arg(short, long, default_value = "auto")]
    pub detect_workspace: DetectWorkspace,

    #[arg(long)]
    pub filter: Option<String>,
}

#[derive(ValueEnum, Clone)]
pub enum OutputFormat {
    C4,
    Graphviz,
}

#[derive(ValueEnum, Clone)]
pub enum DetectWorkspace {
    Yes,
    No,
    Auto,
}
