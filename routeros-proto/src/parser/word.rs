use super::{core::take_bytes, error::ParseError, length::parse_length};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserWord<'a> {
    Command(&'a str),
    Reply(&'a str),
    Attribute(&'a str),
    API(&'a str),
    Query(&'a str),
    ZeroLength,
}

pub(crate) fn parse_word(input: &[u8]) -> Result<(&[u8], ParserWord), ParseError> {
    let (input, length) = parse_length(input)?;
    if length == 0 {
        return Ok((input, ParserWord::ZeroLength));
    }
    let (input, word) = take_bytes(input, length as usize)?;
    let word = core::str::from_utf8(word)?;
    let word = match &word[..1] {
        "/" => Ok(ParserWord::Command(word)),
        "!" => Ok(ParserWord::Reply(word)),
        "=" => Ok(ParserWord::Attribute(word)),
        "?" => Ok(ParserWord::Query(word)),
        "." => Ok(ParserWord::API(word)),
        _ => Err(ParseError::UnknownWordToken),
    }?;
    Ok((input, word))
}

#[cfg(test)]
mod tests {
    use super::{parse_word, ParseError, ParserWord};
    use alloc::vec;

    #[test]
    fn test_parse_word() {
        let test_cases = vec![
            (vec![0], ParserWord::ZeroLength),
            (
                vec![0x06, 0x2f, 0x6c, 0x6f, 0x67, 0x69, 0x6e],
                ParserWord::Command("/login"),
            ),
            (
                vec![0x05, 0x21, 0x64, 0x6f, 0x6e, 0x65],
                ParserWord::Reply("!done"),
            ),
            (
                vec![0x08, 0x3f, 0x66, 0x6f, 0x6f, 0x3d, 0x62, 0x61, 0x72],
                ParserWord::Query("?foo=bar"),
            ),
            (
                vec![
                    0x0c, 0x3d, 0x63, 0x6f, 0x6d, 0x6d, 0x65, 0x6e, 0x74, 0x3d, 0x66, 0x6f, 0x6f,
                ],
                ParserWord::Attribute("=comment=foo"),
            ),
            (
                vec![0x08, 0x2e, 0x74, 0x61, 0x67, 0x3d, 0x78, 0x79, 0x7a],
                ParserWord::API(".tag=xyz"),
            ),
        ];
        for (input, expected_word) in test_cases.into_iter() {
            let (_, word) = parse_word(input.as_slice()).unwrap();
            assert_eq!(word, expected_word);
        }
    }

    #[test]
    fn test_parse_word_invalid() {
        assert_eq!(
            parse_word(&[0x08, 0x2d, 0x74, 0x61, 0x67, 0x3d, 0x78, 0x79, 0x7a]),
            Err(ParseError::UnknownWordToken)
        );
        assert_eq!(parse_word(&[]), Err(ParseError::IncompleteInput(1)));
        assert_eq!(parse_word(&[0x01]), Err(ParseError::IncompleteInput(1)));
    }
}
