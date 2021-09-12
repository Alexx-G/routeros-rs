use core::str::Utf8Error;
#[cfg(not(feature = "std"))]
use core::fmt::{Formatter, Display, Result};

#[cfg(feature = "std")]
use thiserror::Error;

#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Error, Debug))]
pub enum ParseError {
    #[cfg_attr(feature = "std", error("The input is missing {0} bytes"))]
    IncompleteInput(usize),
    #[cfg_attr(feature = "std", error("Cannot decode word length"))]
    InvalidWordLength,
    #[cfg(not(feature = "std"))]
    InvalidStringEncoding,
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "std", error("Failed to decode UTF-8"))]
    InvalidStringEncoding {
        #[from]
        source: Utf8Error,
    },
    #[cfg_attr(feature = "std", error("The word starts with unknown word token"))]
    UnknownWordToken,
    #[cfg_attr(feature = "std", error("The sentence starts with unexpected control word"))]
    UnexpectedControlWord,
    #[cfg_attr(feature = "std", error("The sentence contains more than one control words"))]
    MoreThanOneControlWords,
    #[cfg_attr(feature = "std", error("Failed to decode attribute - invalid format"))]
    InvalidAttributeFormat,
    #[cfg_attr(feature = "std", error("The sentence contains unsupported API attribute"))]
    UnsupportedAPIAttribute,
    #[cfg_attr(feature = "std", error("Reply sentence contains unsupported attribute"))]
    UnsupportedReplyAttribute,
}

#[cfg(not(feature = "std"))]
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "ParseError({})", self) 
    }
}


#[cfg(not(feature = "std"))]
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
       Self::InvalidStringEncoding
    }
}
