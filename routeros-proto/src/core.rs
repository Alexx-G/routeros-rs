use alloc::{string::String, vec::Vec};

use crate::error::ParseError;

pub trait Decodable {
    fn from_bytes_slice(input: &[u8]) -> Result<Self, ParseError>
    where
        Self: Sized;
}

pub trait Encodable {
    fn to_bytes_vec(&self) -> Vec<u8>;
}

#[derive(Debug)]
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
