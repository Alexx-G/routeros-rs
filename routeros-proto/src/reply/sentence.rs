use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use crate::{
    core::{Attribute, Decodable, Encodable},
    encoder,
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
    /// Create a new [builder][crate::reply::ReplyBuilder] instance to construct a reply.
    pub fn builder(reply: ReplyWord) -> ReplyBuilder {
        ReplyBuilder::new(reply)
    }
}

impl Encodable for Reply {
    fn to_bytes_vec(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(256);
        encoder::encode_str((&self.reply).into(), &mut buffer);
        encoder::encode_attributes(self.attributes.iter(), &mut buffer);
        encoder::encode_tag(self.tag.as_ref().map(|v| v.as_str()), &mut buffer);
        buffer.push(0);
        buffer
    }
}

impl Decodable for Reply {
    fn from_bytes_slice(input: &[u8]) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        let (_, words) = parse_sentence(input)?;
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
        Ok(Self {
            reply: reply_word,
            attributes,
            tag,
        })
    }
}
