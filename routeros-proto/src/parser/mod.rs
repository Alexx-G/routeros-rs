mod core;
mod length;
mod sentence;
mod tokens;
mod word;

pub(crate) use sentence::parse_sentence;
pub(crate) use word::{ParserAPIAttribute, ParserWord};
