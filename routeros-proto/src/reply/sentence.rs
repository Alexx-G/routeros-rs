use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use crate::parser::{parse_sentence, ParseError, ParserWord};

use super::ReplyWord;

#[derive(Debug)]
pub struct Reply {
    pub reply: ReplyWord,
    pub attributes: Vec<String>,
    pub tag: Option<String>,
}

impl Reply {
    pub fn from_bytes(input: &[u8]) -> Result<(&[u8], Self), ParseError> {
        let (input, words) = parse_sentence(input)?;
        let reply_word = match &words[0] {
            ParserWord::Reply(r) => Ok(ReplyWord::from(*r)),
            _ => Err(ParseError::UnexpectedControlWord),
        }?;
        let (tag, attributes) = if words.len() == 1 {
            (None, Vec::with_capacity(0))
        } else {
            let mut attributes = Vec::with_capacity(words.len());
            let mut tag = None;
            for word in words[1..].iter() {
                match word {
                    ParserWord::Attribute(v) => {
                        attributes.push(v.to_string());
                        Ok(())
                    }
                    ParserWord::API(v) => {
                        tag = Some(v.to_string());
                        Ok(())
                    }
                    _ => Err(ParseError::UnexpectedControlWord),
                }?;
            }
            (tag, attributes)
        };
        Ok((
            input,
            Self {
                reply: reply_word,
                attributes,
                tag,
            },
        ))
    }
}
