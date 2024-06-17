use crate::core::errors::Error;

#[derive(Debug)]
pub enum ItemsBoxOutput {
    DirectoryLoaded(String),
    Error(Error),
}
