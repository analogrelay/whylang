use tokenizer;

// TODO: We'll want Error IDs and Text Spans
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Error {
    UnexpectedEndOfFile,
    TokenizerError(tokenizer::Error),
}

impl From<tokenizer::Error> for Error {
    fn from(e: tokenizer::Error) -> Error {
        Error::TokenizerError(e)
    }
}