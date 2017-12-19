use text;

#[derive(Debug)]
pub enum Error {
    TextError(text::Error),
}

impl From<text::Error> for Error {
    fn from(e: text::Error) -> Error {
        Error::TextError(e)
    }
}