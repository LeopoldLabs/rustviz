use crate::{
    args::Level,
    c4::{Diagram, Element, ElementKind, Relationship, RelationshipKind},
};
use anyhow::anyhow;
use rustdoc_types::ItemEnum;
use std::collections::{HashMap, HashSet};

pub fn walker(
    max_depth: Option<u32>,
    crates: HashMap<String, rustdoc_types::Crate>,
    level: Level,
) -> Diagram {
    let mut queue;
    let crate_info;
    match level {
        Level::Module { crate_name } => {
            crate_info = crates
                .get(&crate_name)
                .ok_or_else(|| anyhow!("Crate {crate_name} not found"))
                .unwrap();
            queue = HashSet::from([crate_info.root]);
        }
        Level::Crate => {
            todo!()
        }
    }

    let mut depth = 0;

    let mut diagram = Diagram::new("Crate Overview".into());

    let mut visited = HashSet::new();

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
                kind: ElementKind::Container,
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
                        kind: RelationshipKind::Rel,
                    });
                }
            }
        }

        depth += 1;

        queue = upcoming;
    }

    diagram
}
