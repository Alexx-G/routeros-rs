use core::marker::PhantomData;

use alloc::{format, string::String, vec::Vec};

use crate::encoder::encode_length;

use super::word::CommandWord;

/// It's an implementation of the command sentence from [RouterOS API](https://help.mikrotik.com/docs/display/ROS/API).
pub struct Command {
    /// [Command word](https://help.mikrotik.com/docs/display/ROS/API#API-Commandword) - first word from the command
    /// sentence, it starts with '/' and follows CLI structure with space (' ') being replaced with '/'.
    pub command: CommandWord,
    /// [Command attributes](https://help.mikrotik.com/docs/display/ROS/API#API-Attributeword) - a list of parameters
    /// for the command. Each attribute is a key/value pair, with value being optional. Order is not important and should
    /// not be relied on.
    pub attributes: Vec<Attribute>,
    /// [Command query](https://help.mikrotik.com/docs/display/ROS/API#API-Queryword) - a list of query parameters that
    /// restrict scope of the command. The general format is key/value pair, however theare are a few special operators
    /// that do not follow this format.
    pub query: Vec<String>,
    /// [Command tag](https://help.mikrotik.com/docs/display/ROS/API#API-APIattributeword) - a special API Attribute that
    /// allows assigning an identifier to the command, the same identifier will be added to the corresponding reply sentence.
    pub tag: Option<String>,
    #[doc(hidden)]
    pub(crate) phantom: PhantomData<()>,
}

pub struct Attribute {
    pub name: String,
    pub value: Option<String>,
}

impl Attribute {
    pub fn new(name: String, value: Option<String>) -> Self {
        Self { name, value }
    }

    pub fn empty(name: String) -> Self {
        Self { name, value: None }
    }
}

impl Command {
    pub fn to_bytes_vec(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(256);
        let command: &str = (&self.command).into();
        buffer.extend(encode_length(command.len() as u32));
        buffer.extend_from_slice(command.as_bytes());
        for attribute in self.attributes.iter() {
            let value = attribute.value.as_ref().map(|s| s.as_str()).unwrap_or("");
            let attribute = format!("={}={}", attribute.name, value);
            buffer.extend(encode_length(attribute.len() as u32));
            buffer.extend_from_slice(attribute.as_bytes());
        }
        for query in self.query.iter() {
            let query = format!("?{}", query);
            buffer.extend_from_slice(query.as_bytes());
        }
        match &self.tag {
            Some(t) => {
                let tag = format!(".tag={}", t);
                buffer.extend_from_slice(tag.as_bytes());
            }
            None => {}
        };
        buffer.extend_from_slice(&[0, 0]);
        buffer
    }
}
