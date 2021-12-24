use alloc::{string::String, vec::Vec};

use crate::core::Attribute;

use super::{Command, CommandWord};

/// A convenience builder to be used to construct a [Command][Command] sentence programmatically.
///
/// # Examples
///
/// ```
/// use routeros_proto::command::{CommandBuilder, CommandWord};
/// let comand = CommandBuilder::new(CommandWord::Login)
///     .attribute("name", "foo")
///     .build();
/// ```
pub struct CommandBuilder {
    command: CommandWord,
    attributes: Vec<Attribute>,
    query: Vec<String>,
    tag: Option<String>,
}

impl CommandBuilder {
    /// Create a new builder instance using a [command word][CommandWord]
    pub fn new(command: CommandWord) -> Self {
        Self {
            command,
            attributes: Vec::new(),
            query: Vec::new(),
            tag: None,
        }
    }

    /// Create a new builder instance using a raw (String-based) [command word][CommandWord]
    pub fn new_raw<C: Into<String>>(command: C) -> Self {
        Self::new(CommandWord::Raw(command.into()))
    }

    /// Add a new command attribute
    pub fn attribute<N: Into<String>, V: Into<String>>(mut self, name: N, value: V) -> Self {
        self.attributes
            .push(Attribute::new(name.into(), Some(value.into())));
        self
    }

    /// Add a new empty command attribute
    pub fn empty_attribute<S: Into<String>>(mut self, name: S) -> Self {
        self.attributes.push(Attribute::empty(name.into()));
        self
    }

    /// Add a query attribute to the command
    pub fn query<S: Into<String>>(mut self, query: S) -> Self {
        self.query.push(query.into());
        self
    }

    /// Set a tag on the command
    pub fn tag<S: Into<String>>(mut self, tag: Option<S>) -> Self {
        self.tag = tag.map(|s| s.into());
        self
    }

    /// Build a [command sentence][Command] instance, consuming the builder instance.
    pub fn build(self) -> Command {
        Command {
            command: self.command,
            attributes: self.attributes,
            query: self.query,
            tag: self.tag,
        }
    }
}
