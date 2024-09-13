#[derive(Debug)]
pub enum ItemsBoxOutput {
    DirectoryLoaded(String),
    Error(String),
}
