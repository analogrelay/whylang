use std::convert::From;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    /// Indicates that the operation cannot be completed because the end of the file has been reached
    EndOfFile,

    /// Indicates that a unicode decoding error occurred while loading the text
    InvalidText,
}