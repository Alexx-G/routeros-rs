use alloc::string::String;

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
