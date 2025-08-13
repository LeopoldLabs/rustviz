use anyhow::ensure;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::Command,
};

fn target_folder(path: &Path) -> PathBuf {
    path.join("target/doc")
}

fn generate_docs(path: &Path) -> anyhow::Result<()> {
    //! Generates JSON descriptions of a rust workspace
    let dir_path = target_folder(path);

    std::fs::remove_dir_all(&dir_path)?;

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

    Ok(())
}

fn read_docs(path: &Path) -> anyhow::Result<HashMap<String, rustdoc_types::Crate>> {
    //! Reads rustdoc descriptions from the target folder of a given directory.

    let dir_path = target_folder(path);
    let dir = std::fs::read_dir(&dir_path)?;

    let mut crates = HashMap::new();

    for entry in dir {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        if !file_type.is_file() {
            anyhow::bail!("Unexpected entry type, entry: {path:?}")
        }
        let contents = std::fs::read_to_string(path)?;
        let crate_info: rustdoc_types::Crate = serde_json::from_str(&contents)?;
        ensure!(
            crate_info.format_version == rustdoc_types::FORMAT_VERSION,
            "json version matches our rustdoc_types version"
        );

        let crate_name = entry
            .file_name()
            .into_string()
            .map_err(|_| anyhow::anyhow!("couldn't convert filename to UTF-8 string"))?;

        let crate_name = crate_name.strip_suffix(".json").unwrap().to_string();

        crates.insert(crate_name, crate_info);
    }

    Ok(crates)
}

pub fn docs(path: &Path) -> anyhow::Result<HashMap<String, rustdoc_types::Crate>> {
    generate_docs(path)?;
    read_docs(path)
}
