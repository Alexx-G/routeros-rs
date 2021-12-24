use alloc::{format, vec::Vec};

use crate::core::Attribute;

pub(crate) struct EncodedLength {
    encoded: [u8; 5],
    encoded_size: usize,
}

impl EncodedLength {
    pub(crate) fn new(length: usize) -> Self {
        let length = length as u32;
        let mut encoded = [0; 5];
        let encoded_size = if length < 0x80 {
            encoded[0] = length as u8;
            1
        } else if length < 0x4000 {
            let l = (length | 0x8000) as u16;
            encoded[..2].copy_from_slice(&l.to_be_bytes());
            2
        } else if length < 0x200000 {
            let l = length | 0xC00000 as u32;
            encoded[..3].copy_from_slice(&l.to_be_bytes()[1..]);
            3
        } else if length < 0x10000000 {
            let l = length | 0xE0000000;
            encoded[..4].copy_from_slice(&l.to_be_bytes());
            4
        } else {
            encoded[0] = 0xF0;
            encoded[1..].copy_from_slice(&length.to_be_bytes());
            5
        };
        Self {
            encoded,
            encoded_size,
        }
    }

    pub(crate) fn as_slice(&self) -> &[u8] {
        &self.encoded[..self.encoded_size]
    }
}

pub(crate) fn encode_str(input: &str, buffer: &mut Vec<u8>) {
    buffer.extend(EncodedLength::new(input.len()).as_slice());
    buffer.extend_from_slice(input.as_bytes());
}

pub(crate) fn encode_attributes<'a, I>(attributes: I, buffer: &mut Vec<u8>)
where
    I: Iterator<Item = &'a Attribute>,
{
    for attribute in attributes {
        let value = attribute.value.as_ref().map(|s| s.as_str()).unwrap_or("");
        let attribute = format!("={}={}", attribute.name, value);
        encode_str(attribute.as_str(), buffer);
    }
}

pub(crate) fn encode_tag(tag: Option<&str>, buffer: &mut Vec<u8>) {
    match tag {
        Some(t) => {
            let tag = format!(".tag={}", t);
            encode_str(tag.as_str(), buffer);
        }
        None => {}
    };
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use crate::encoder::EncodedLength;

    #[test]
    fn test_encode_length() {
        let test_cases = [
            (0x00000000, vec![0x00]),
            (0x00000001, vec![0x01]),
            (0x00000087, vec![0x80, 0x87]),
            (0x00004321, vec![0xC0, 0x43, 0x21]),
            (0x002acdef, vec![0xE0, 0x2a, 0xcd, 0xef]),
            (0x10000080, vec![0xF0, 0x10, 0x00, 0x00, 0x80]),
        ];
        for (length, expected_bytes) in test_cases.iter() {
            let encoded_length = EncodedLength::new(*length);
            assert_eq!(expected_bytes.as_slice(), encoded_length.as_slice());
        }
    }
}
