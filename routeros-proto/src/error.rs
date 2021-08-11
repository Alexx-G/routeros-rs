use core::str::Utf8Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    IncompleteInput(usize),
    InvalidWordLength,
    InvalidStringEncoding,
    UnknownWordToken,
    UnexpectedControlWord,
    MoreThanOneControlWords,
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidStringEncoding
    }
}
