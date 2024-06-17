use std::io::ErrorKind;

#[derive(Debug)]
pub enum Error {
    IoError(ErrorKind),
    OpenFileError(String, ErrorKind),
}
