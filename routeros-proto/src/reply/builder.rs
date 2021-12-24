use alloc::{string::String, vec::Vec};

use crate::core::Attribute;

use super::{Reply, ReplyWord};

/// A convenience builder to be used to construct a [Reply][Reply] sentence programmatically.
///
/// # Examples
///
/// ```
/// use routeros_proto::reply::{ReplyBuilder, ReplyWord};
/// let comand = ReplyBuilder::new(ReplyWord::Data)
///     .attribute("name", "foo")
///     .build();
/// ```
pub struct ReplyBuilder {
    reply: ReplyWord,
    attributes: Vec<Attribute>,
    tag: Option<String>,
}

impl ReplyBuilder {
    /// Create a new builder instance using a [reply word][ReplyWord]
    pub fn new(reply: ReplyWord) -> Self {
        Self {
            reply,
            attributes: Vec::new(),
            tag: None,
        }
    }

    /// Add a new reply attribute
    pub fn attribute<N: Into<String>, V: Into<String>>(mut self, name: N, value: V) -> Self {
        self.attributes
            .push(Attribute::new(name.into(), Some(value.into())));
        self
    }

    /// Add a new empty reply attribute
    pub fn empty_attribute<S: Into<String>>(mut self, name: S) -> Self {
        self.attributes.push(Attribute::empty(name.into()));
        self
    }

    /// Set a tag on the reply
    pub fn tag<S: Into<String>>(mut self, tag: Option<S>) -> Self {
        self.tag = tag.map(|s| s.into());
        self
    }

    /// Build a [reply sentence][Reply] instance and consumes the builder instance.
    pub fn build(self) -> Reply {
        Reply {
            reply: self.reply,
            attributes: self.attributes,
            tag: self.tag,
        }
    }
}
