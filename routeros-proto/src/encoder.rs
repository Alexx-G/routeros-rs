use alloc::vec::Vec;

pub fn encode_length(length: u32) -> Vec<u8> {
    let mut result = Vec::with_capacity(5);
    if length < 0x80 {
        result.push(length as u8);
    } else if length < 0x4000 {
        let l = (length | 0x8000) as u16;
        result.extend_from_slice(&l.to_be_bytes());
    } else if length < 0x200000 {
        let l = length | 0xC00000 as u32;
        result.extend_from_slice(&l.to_be_bytes()[1..]);
    } else if length < 0x10000000 {
        let l = length | 0xE0000000;
        result.extend_from_slice(&l.to_be_bytes());
    } else {
        result.push(0xF0);
        result.extend_from_slice(&length.to_be_bytes());
    };
    result
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use crate::encoder::encode_length;

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
            let encoded_length = encode_length(*length);
            assert_eq!(expected_bytes.as_slice(), encoded_length.as_slice());
        }
    }
}
