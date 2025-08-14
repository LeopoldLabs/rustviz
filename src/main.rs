#![feature(exit_status_error)]

use std::path::PathBuf;

use anyhow::{Result, bail};
use cargo_toml::Manifest;
use clap::Parser;

mod args;
mod diagram;
mod package;
mod rustdoc;
mod workspace;

use args::{Args, DetectWorkspace};

use crate::diagram::Diagram;

fn main() -> Result<()> {
    let args = Args::try_parse()?;

    let manifest_path = args.project_path.join("Cargo.toml");
    let manifest = Manifest::from_path(&manifest_path)?;

    let mut diagram = Diagram::new("Diagram".to_string());
    
    match args.detect_workspace {
        DetectWorkspace::Yes => {
            let Some(workspace) = manifest.workspace else {
                bail!("No workspace found");
            };

            workspace::parse_workspace(&mut diagram, &args.project_path, &workspace)?;
        }
        DetectWorkspace::No => {
            let Some(package) = manifest.package else {
                bail!("No package found");
            };
            package::parse_package(&mut diagram, &package)?;
        }
        DetectWorkspace::Auto => {
            if let Some(workspace) = manifest.workspace {
                workspace::parse_workspace(&mut diagram, &args.project_path, &workspace)?;
            } else {
                let Some(package) = manifest.package else {
                    bail!("No package found");
                };
                package::parse_package(&mut diagram, &package)?;
            };
        }
    }

    let output = diagram.render(args.output_format)?;

    if args.output_path == PathBuf::from("-") {
        println!("{output}");
    } else {
        std::fs::write(args.output_path, output)?;
    }

    Ok(())
}
