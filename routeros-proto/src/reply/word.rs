use alloc::string::{String, ToString};

use super::constants::{DATA_REPLY, DONE_REPLY, FATAL_REPLY, TRAP_REPLY};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplyWord {
    Done,
    Error,
    Fatal,
    Data,
    Other(String),
}

impl From<&str> for ReplyWord {
    fn from(value: &str) -> Self {
        match value {
            DONE_REPLY => ReplyWord::Done,
            DATA_REPLY => ReplyWord::Data,
            TRAP_REPLY => ReplyWord::Error,
            FATAL_REPLY => ReplyWord::Fatal,
            v => ReplyWord::Other(v.to_string()),
        }
    }
}

impl<'a> From<&'a ReplyWord> for &'a str {
    fn from(value: &'a ReplyWord) -> &'a str {
        match value {
            ReplyWord::Done => DONE_REPLY,
            ReplyWord::Error => TRAP_REPLY,
            ReplyWord::Fatal => FATAL_REPLY,
            ReplyWord::Data => DATA_REPLY,
            ReplyWord::Other(v) => v.as_str(),
        }
    }
}
