use super::error::ParseError;

pub(crate) fn take_bytes(input: &[u8], count: usize) -> Result<(&[u8], &[u8]), ParseError> {
    if input.len() < count {
        let needed = count - input.len();
        Err(ParseError::IncompleteInput(needed))
    } else {
        Ok((&input[count..], &input[..count]))
    }
}

pub(crate) fn take_u8(input: &[u8]) -> Result<(&[u8], u8), ParseError> {
    let (input, result) = take_bytes(input, 1)?;
    return Ok((input, result[0]));
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use alloc::vec::Vec;

    use crate::parser::error::ParseError;

    use super::{take_bytes, take_u8};

    #[test]
    fn test_take_bytes() {
        let test_cases: Vec<(Vec<u8>, usize, Vec<u8>, Vec<u8>)> = vec![
            (vec![], 0, vec![], vec![]),
            (vec![1, 2], 1, vec![2], vec![1]),
            (vec![1, 2, 3], 2, vec![3], vec![1, 2]),
            (vec![1, 2, 3], 3, vec![], vec![1, 2, 3]),
        ];
        for (input, length, remaining_input, expected_data) in test_cases.into_iter() {
            let (input, data) = take_bytes(input.as_slice(), length).unwrap();
            assert_eq!(
                data.len(),
                length,
                "Expected length of consumed input doesn't match"
            );
            assert_eq!(
                input,
                remaining_input.as_slice(),
                "Remaining input doesn't match"
            );
            assert_eq!(
                data,
                expected_data.as_slice(),
                "Consumed input doesn't match"
            );
        }
    }

    #[test]
    fn test_take_bytes_incomplete() {
        assert_eq!(Err(ParseError::IncompleteInput(1)), take_bytes(&[], 1));
        assert_eq!(Err(ParseError::IncompleteInput(3)), take_bytes(&[1, 2], 5));
    }

    #[test]
    fn test_take_u8() {
        assert_eq!(Ok((vec![0, 1].as_slice(), 42)), take_u8(&[42, 0, 1]));
    }
}
