use crate::diagram::{self, Diagram, Element, ElementKind};
use anyhow::{Result, ensure};
use rustdoc_types::{ItemEnum, Visibility};
use std::{
    collections::{HashMap, VecDeque},
    path::{Path, PathBuf},
    process::Command,
};

fn id_to_id(id: rustdoc_types::Id) -> diagram::Id {
    id.0.to_string()
}

pub fn parse_package(
    diagram: &mut Diagram,
    package_path: &Path,
    _manifest: &cargo_toml::Manifest,
) -> Result<()> {
    generate_docs(&package_path)?;
    let crate_info = read_docs(&package_path.join("target/doc"))?;

    let root_item = &crate_info.index[&crate_info.root];
    diagram.add_element(Element {
        id: id_to_id(crate_info.root),
        title: root_item.name.as_ref().unwrap().clone(),
        description: root_item.docs.as_ref().unwrap().clone(),
        kind: ElementKind::Container,
    });

    let ItemEnum::Module(root_module) = &root_item.inner else {
        anyhow::bail!("Root item is not a module");
    };

    let mut queue = VecDeque::from_iter(
        root_module
            .items
            .iter()
            .map(|item_id| (crate_info.root, item_id)),
    );

    while let Some((parent_id, id)) = queue.pop_front() {
        let element = crate_info.index.get(&id).unwrap();
        let id_s = id_to_id(*id);


        if element.visibility != Visibility::Public || diagram.has_element(&id_s) {
            continue;
        }

        let ItemEnum::Module(module) = &element.inner else {
            continue;
        };

        let element = Element {
            id: id_s.clone(),
            title: element.name.as_ref().unwrap().to_string(),
            description: "Module".to_string(),
            kind: ElementKind::Container,
        };

        diagram.add_element(element);
        diagram.add_relationship(id_to_id(parent_id), id_s);

        for child in module.items.iter() {
            queue.push_back((*id, child));
        }
    }

    Ok(())
}

fn generate_docs(path: &Path) -> anyhow::Result<()> {
    //! Generates JSON descriptions of a rust workspace
    let deleted = std::fs::remove_dir_all(&path.join("target/doc"));
    if let Err(e) = deleted {
        if e.kind() != std::io::ErrorKind::NotFound {
            return Err(e.into());
        }
    }

    println!("cargo doc");
    Command::new("cargo")
        .args([
            "+nightly",
            "doc",
            "--document-private-items",
            "--workspace",
            "--no-deps",
        ])
        .env("RUSTDOCFLAGS", "-Z unstable-options --output-format json")
        .output()?
        .exit_ok()?;

    println!("cargo doc done");

    Ok(())
}

fn read_docs(path: &Path) -> anyhow::Result<rustdoc_types::Crate> {
    //! Reads rustdoc descriptions from the target folder of a given directory.

    println!("{}", path.display());
    let dir = std::fs::read_dir(&path)?;
    println!("done");

    let entry = dir.into_iter().next().expect("No entry found")?;
    if !entry.file_type()?.is_file() {
        anyhow::bail!("Unexpected entry type, entry: {path:?}")
    }

    let contents = std::fs::read_to_string(path)?;
    let crate_info: rustdoc_types::Crate = serde_json::from_str(&contents)?;

    Ok(crate_info)
}
