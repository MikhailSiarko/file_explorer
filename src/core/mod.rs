pub(crate) mod errors;

use std::path::PathBuf;

use errors::Error;

pub async fn load_items(current_dir: &str) -> Result<Vec<PathBuf>, Error> {
    match tokio::fs::read_dir(&current_dir).await {
        Ok(mut entries) => {
            let mut items = Vec::new();
            while let Ok(option) = entries.next_entry().await {
                match option {
                    Some(item) => items.push(item.path()),
                    None => break,
                }
            }
            Ok(items)
        }
        Err(error) => Err(Error::IoError(error.kind())),
    }
}
