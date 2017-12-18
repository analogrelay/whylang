use std::{io, mem};
use std::convert::From;

#[derive(Debug)]
pub enum Error {
    /// Indicates an unexpected I/O error occurred
    IoError(io::Error),

    /// Indicates that the operation cannot be completed because the end of the file has been reached
    EndOfFile,

    /// Indicates that a unicode decoding error occurred while loading the text
    InvalidText,
}

// We implement PartialEq, because IoError != IoError (because io::Error is not comparable)
impl PartialEq<Error> for Error {
    fn eq(&self, other: &Error) -> bool {
        match (self, other) {
            (&Error::IoError(_), _) => false,
            (_, &Error::IoError(_)) => false,
            (x, y) => mem::discriminant(x) == mem::discriminant(y),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IoError(e)
    }
}