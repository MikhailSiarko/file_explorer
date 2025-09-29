use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub fn load_items(current_dir: &str) -> Result<Vec<PathBuf>> {
    let mut dir = fs::read_dir(current_dir).context("Error while open directory")?;
    let mut items = Vec::new();
    while let Some(Ok(item)) = dir.next() {
        items.push(item.path());
    }

    Ok(items)
}

pub fn open_item(path: &str) -> Result<()> {
    open::that(&path).context("Error while opening the file")
}
