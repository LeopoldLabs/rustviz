use std::collections::HashSet;

use anyhow::{Result, ensure};
use clap::Parser;

mod args;
use args::Args;
use rustdoc_types::{ItemEnum, Visibility};

use crate::c4::{Diagram, Element, Relationship};

mod c4;

fn main() -> Result<()> {
    let args = Args::try_parse()?;

    let file = std::fs::read_to_string(args.filepath)?;

    let crate_info: rustdoc_types::Crate = serde_json::from_str(&file)?;

    ensure!(
        crate_info.format_version == rustdoc_types::FORMAT_VERSION,
        "json version matches our rustdoc_types version"
    );

    let diagram = build_diagram(&crate_info);

    println!("{diagram}");

    Ok(())
}

fn build_diagram(crate_info: &rustdoc_types::Crate) -> Diagram {
    let mut diagram = Diagram::new("Crate Overview".into());

    let mut visited = HashSet::new();
    let mut queue = HashSet::from([crate_info.root]);

    while !queue.is_empty() {
        let mut upcoming = HashSet::new();
        for id in queue.iter() {
            visited.insert(*id);

            let item = &crate_info.index[id];

            let ItemEnum::Module(module) = &item.inner else {
                continue;
            };

            diagram.add_element(Element {
                id: *id,
                kind: c4::ElementKind::Container,
                title: item.name.clone().unwrap(),

                // this is cursed but hey
                description: item.span.clone().unwrap().filename.to_str().unwrap().into(),
            });

            for child in module.items.iter() {
                if !matches!(&crate_info.index[child].inner, ItemEnum::Module(_)) {
                    continue;
                }

                if !visited.contains(child) {
                    upcoming.insert(*child);

                    diagram.add_relationship(Relationship {
                        from: *id,
                        to: *child,
                        description: "Contains".into(),
                        kind: c4::RelationshipKind::Rel,
                    });
                }
            }
        }

        queue = upcoming;
    }

    diagram
}
