use std::path::Path;

use crate::diagram::{Diagram, Element, ElementKind};
use anyhow::bail;

pub fn parse_workspace(diagram: &mut Diagram, workspace_path: &Path, workspace: &cargo_toml::Workspace) -> anyhow::Result<()> {

    for member in &workspace.members {
        let manifest_path = workspace_path.join(member).join("Cargo.toml");
        let manifest = cargo_toml::Manifest::from_path(&manifest_path)?;
        
        let Some(_package) = manifest.package else {
            bail!("No package found in member {member}");
        };
        
        let dependencies = manifest.dependencies;
        
        for (name, dependency) in dependencies {
            if dependency.is_crates_io() {
                continue;
            }

            let name = dependency.package().unwrap_or(&name);
            if workspace.members.contains(&name.to_string()) {
                diagram.add_element(Element {
                    kind: ElementKind::Container,
                    id: name.to_string(),
                    title: name.to_string(),
                    description: "".to_string(),
                });

                diagram.add_relationship(member.to_string(), name.to_string());
            }
        }
    }

    Ok(())
}