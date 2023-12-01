use std::error::Error;
use std::io;
use std::path::{Path, PathBuf};

pub fn io_invalid() -> io::Error {
    io::ErrorKind::InvalidData.into()
}

pub fn io_invalid_with<E: Error + Send + Sync + 'static>(e: E) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, e)
}

pub fn data_dir() -> PathBuf {
    Path::new(file!()).ancestors().nth(2).unwrap().join("data")
}
