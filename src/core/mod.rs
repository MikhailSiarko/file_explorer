use anyhow::{Context, Result};
use std::path::PathBuf;

pub async fn load_items(current_dir: &str) -> Result<Vec<PathBuf>> {
    let mut entries = tokio::fs::read_dir(&current_dir)
        .await
        .context("Failed to read directory")?;
    let mut items = Vec::new();
    while let Some(item) = entries
        .next_entry()
        .await
        .context("Failed to read a directory item")?
    {
        items.push(item.path());
    }
    Ok(items)
}
