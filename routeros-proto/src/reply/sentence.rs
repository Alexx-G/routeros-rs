use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

use crate::{
    core::Attribute,
    encoder::EncodedLength,
    error::ParseError,
    parser::{parse_sentence, ParserAPIAttribute, ParserWord},
};

use super::{builder::ReplyBuilder, ReplyWord};

#[derive(Debug)]
pub struct Reply {
    pub reply: ReplyWord,
    pub attributes: Vec<Attribute>,
    pub tag: Option<String>,
}

impl Reply {
    pub fn builder(reply: ReplyWord) -> ReplyBuilder {
        ReplyBuilder::new(reply)
    }

    pub fn to_bytes_vec(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(256);
        let command: &str = (&self.reply).into();
        buffer.extend_from_slice(EncodedLength::new(command.len()).as_slice());
        buffer.extend_from_slice(command.as_bytes());
        for attribute in self.attributes.iter() {
            let value = attribute.value.as_ref().map(|s| s.as_str()).unwrap_or("");
            let attribute = format!("={}={}", attribute.name, value);
            buffer.extend(EncodedLength::new(attribute.len()).as_slice());
            buffer.extend_from_slice(attribute.as_bytes());
        }
        match &self.tag {
            Some(t) => {
                let tag = format!(".tag={}", t);
                buffer.extend_from_slice(tag.as_bytes());
            }
            None => {}
        };
        buffer.push(0);
        buffer
    }

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
                    ParserWord::Attribute((name, value)) => {
                        attributes.push(Attribute::new(
                            name.to_string(),
                            value.map(|v| v.to_string()),
                        ));
                        Ok(())
                    }
                    ParserWord::API(attr) => {
                        match attr {
                            &ParserAPIAttribute::Tag(v) => {
                                tag = Some(v.to_string());
                            }
                        };
                        Ok(())
                    }
                    _ => Err(ParseError::UnsupportedReplyAttribute),
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
