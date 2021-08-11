use core::convert::TryInto;

use crate::error::ParseError;

use super::core::{take_bytes, take_u8};

pub(crate) fn parse_length(input: &[u8]) -> Result<(&[u8], u32), ParseError> {
    let (input, length) = take_u8(input)?;
    let (input, length) = if length & 0x80 == 0 {
        (input, length as u32)
    } else if length & 0xC0 == 0x80 {
        let first_byte = length & !0xC0;
        let (input, lower_byte) = take_u8(input)?;
        (input, u32::from_be_bytes([0, 0, first_byte, lower_byte]))
    } else if length & 0xE0 == 0xC0 {
        let first_byte = length & !0xE0;
        let (input, lower_bytes) = take_bytes(input, 2)?;
        (
            input,
            u32::from_be_bytes([0, first_byte, lower_bytes[0], lower_bytes[1]]),
        )
    } else if length & 0xF0 == 0xE0 {
        let first_byte = length & !0xF0;
        let (input, lower_bytes) = take_bytes(input, 3)?;
        (
            input,
            u32::from_be_bytes([first_byte, lower_bytes[0], lower_bytes[1], lower_bytes[2]]),
        )
    } else if length & 0xF8 == 0xF0 {
        let (input, length_bytes) = take_bytes(input, 4)?;
        (input, u32::from_be_bytes(length_bytes.try_into().unwrap()))
    } else {
        return Err(ParseError::InvalidWordLength);
    };

    Ok((input, length as u32))
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use alloc::vec::Vec;

    use crate::parser::length::parse_length;

    #[test]
    fn test_word_length_parsing() {
        let length_test_cases: Vec<(u32, Vec<u8>)> = vec![
            (0x00000000, vec![0x00]),
            (0x00000001, vec![0x01]),
            (0x00000087, vec![0x80, 0x87]),
            (0x00004321, vec![0xC0, 0x43, 0x21]),
            (0x002acdef, vec![0xE0, 0x2a, 0xcd, 0xef]),
            (0x10000080, vec![0xF0, 0x10, 0x00, 0x00, 0x80]),
        ];
        for (expected_length, input) in length_test_cases {
            let parsed = parse_length(&input);
            assert!(parsed.is_ok(), "Expected parsing to be successful");
            let (_, length) = parsed.unwrap();
            assert_eq!(expected_length, length);
        }
    }
}
