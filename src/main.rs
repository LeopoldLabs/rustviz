#![feature(exit_status_error)]

use std::collections::HashSet;

use anyhow::Result;
use clap::Parser;

mod args;
use args::Args;
use rustdoc_types::ItemEnum;

mod rustdoc;

use crate::c4::{Diagram, Element, Relationship};

mod c4;

fn main() -> Result<()> {
    let args = Args::try_parse()?;

    let crates = rustdoc::docs(&args.project_path);

    Ok(())
}

fn build_diagram(crate_info: &rustdoc_types::Crate, max_depth: Option<u32>) -> Diagram {
    let mut depth = 0;

    let mut diagram = Diagram::new("Crate Overview".into());

    let mut visited = HashSet::new();
    let mut queue = HashSet::from([crate_info.root]);

    while !queue.is_empty() && max_depth.is_none_or(|m| depth < m) {
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

        depth += 1;

        queue = upcoming;
    }

    diagram
}
