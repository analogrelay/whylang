use std::num::ParseIntError;

use text;

#[derive(Debug)]
pub enum Error {
    TextError(text::Error),
    ParseIntError(ParseIntError)
}

impl From<text::Error> for Error {
    fn from(e: text::Error) -> Error {
        Error::TextError(e)
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Error {
        Error::ParseIntError(e)
    }
}