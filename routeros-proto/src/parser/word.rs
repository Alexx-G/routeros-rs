use crate::error::ParseError;

use super::{core::take_bytes, length::parse_length, tokens};

pub(crate) type ParserAttribute<'a> = (&'a str, Option<&'a str>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ParserAPIAttribute<'a> {
    Tag(&'a str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ParserWord<'a> {
    Command(&'a str),
    Reply(&'a str),
    Attribute(ParserAttribute<'a>),
    API(ParserAPIAttribute<'a>),
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
    let token = &word[..1];
    let word = match token {
        tokens::FORWARD_SLASH => Ok(ParserWord::Command(word)),
        tokens::EXCLAMATION_MARK => Ok(ParserWord::Reply(word)),
        tokens::EQUALS_SIGN => {
            let attribute = parse_attribute(&word[1..])?;
            Ok(ParserWord::Attribute(attribute))
        }
        tokens::DOT => {
            let attribte = parse_api_attribute(&word[1..])?;
            Ok(ParserWord::API(attribte))
        }
        tokens::QUESTION_MARK => Ok(ParserWord::Query(word)),
        _ => Err(ParseError::UnknownWordToken(token.chars().next().unwrap())),
    }?;
    Ok((input, word))
}

#[inline]
fn parse_api_attribute(word: &str) -> Result<ParserAPIAttribute, ParseError> {
    let sep = word
        .find(tokens::EQUALS_SIGN)
        .ok_or(ParseError::InvalidAttributeFormat)?;
    if sep == 0 || sep == word.len() - 1 {
        return Err(ParseError::InvalidAttributeFormat);
    }

    let attribute = match &word[..sep] {
        tokens::TAG_WORD => Ok(ParserAPIAttribute::Tag(&word[sep + 1..])),
        _ => Err(ParseError::UnsupportedAPIAttribute),
    }?;
    Ok(attribute)
}

#[inline]
fn parse_attribute(word: &str) -> Result<ParserAttribute, ParseError> {
    let sep = word
        .find(tokens::EQUALS_SIGN)
        .ok_or(ParseError::InvalidAttributeFormat)?;
    if sep == 0 {
        return Err(ParseError::InvalidAttributeFormat);
    }
    let value = if sep == word.len() - 1 {
        None
    } else {
        Some(&word[sep + 1..])
    };
    Ok((&word[..sep], value))
}

#[cfg(test)]
mod tests {
    use crate::parser::word::{parse_attribute, ParserAPIAttribute};

    use super::{parse_api_attribute, parse_word, ParseError, ParserWord};
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
                ParserWord::Attribute(("comment", Some("foo"))),
            ),
            (
                vec![0x08, 0x2e, 0x74, 0x61, 0x67, 0x3d, 0x78, 0x79, 0x7a],
                ParserWord::API(ParserAPIAttribute::Tag("xyz")),
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
            Err(ParseError::UnknownWordToken('-'))
        );
        assert_eq!(parse_word(&[]), Err(ParseError::IncompleteInput(1)));
        assert_eq!(parse_word(&[0x01]), Err(ParseError::IncompleteInput(1)));
    }

    #[test]
    fn test_parse_attribute() {
        let test_cases = vec![
            ("foo=bar", ("foo", Some("bar"))),
            ("foo=bar=baz", ("foo", Some("bar=baz"))),
            ("foo=", ("foo", None)),
        ];
        for (input, expected_attribute) in test_cases.into_iter() {
            let attribute = parse_attribute(input).unwrap();
            assert_eq!(attribute, expected_attribute);
        }
    }

    #[test]
    fn test_parse_invalid_attribute() {
        let test_cases = vec!["", "==", "==foo", "foo"];
        for input in test_cases.into_iter() {
            let attribute = parse_attribute(input);
            assert_eq!(attribute, Err(ParseError::InvalidAttributeFormat));
        }
    }

    #[test]
    fn test_parse_api_attribute() {
        let attribute = parse_api_attribute("tag=foo").unwrap();
        assert_eq!(attribute, ParserAPIAttribute::Tag("foo"));
        let attribute = parse_api_attribute("tag=foo=bar").unwrap();
        assert_eq!(attribute, ParserAPIAttribute::Tag("foo=bar"));
    }

    #[test]
    fn test_parse_invalid_api_attribute() {
        let test_cases = vec![
            ("foo=", Err(ParseError::InvalidAttributeFormat)),
            ("foo", Err(ParseError::InvalidAttributeFormat)),
            ("foo=bar", Err(ParseError::UnsupportedAPIAttribute)),
        ];
        for (input, err) in test_cases.into_iter() {
            let attribute = parse_api_attribute(input);
            assert_eq!(attribute, err);
        }
    }
}
