use std::fmt;
use std::io;
use thiserror::Error;

pub type WeaveResult<T> = Result<T, WeaveError>;

#[derive(Error, Debug)]
pub enum WeaveError {
    Io(#[from] io::Error),
}

impl fmt::Display for WeaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WeaveError::Io(err) => writeln!(f, "IO Error: {}", err),
        }
    }
}
