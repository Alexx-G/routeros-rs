use alloc::vec::Vec;

use crate::error::ParseError;

use super::word::{parse_word, ParserWord};

pub(crate) fn parse_sentence(input: &[u8]) -> Result<(&[u8], Vec<ParserWord>), ParseError> {
    let (mut input, word) = parse_word(input)?;
    let word = match word {
        ParserWord::Command(v) => Ok(ParserWord::Command(v)),
        ParserWord::Reply(v) => Ok(ParserWord::Reply(v)),
        _ => Err(ParseError::UnexpectedControlWord),
    }?;
    let mut words = Vec::with_capacity(1);
    words.push(word);
    loop {
        let (remaining_input, word) = parse_word(input)?;
        input = remaining_input;
        let word = match word {
            ParserWord::ZeroLength => break,
            ParserWord::Command(_) | ParserWord::Reply(_) => {
                Err(ParseError::MoreThanOneControlWords)
            }
            _ => Ok(word),
        }?;
        words.push(word);
    }
    Ok((input, words))
}

#[cfg(test)]
mod tests {
    use crate::parser::{parse_sentence, ParserWord};
    use alloc::vec;

    #[test]
    fn test_parse_sentence() {
        let test_cases = vec![
            (
                vec![0x06, 0x2f, 0x6c, 0x6f, 0x67, 0x69, 0x6e, 0x00],
                vec![ParserWord::Command("/login")],
            ),
            (
                vec![0x05, 0x21, 0x64, 0x6f, 0x6e, 0x65, 0x00],
                vec![ParserWord::Reply("!done")],
            ),
            (
                vec![
                    0x04, 0x2f, 0x66, 0x6f, 0x6f, 0x08, 0x2e, 0x74, 0x61, 0x67, 0x3d, 0x78, 0x79,
                    0x7a, 0x0c, 0x3d, 0x63, 0x6f, 0x6d, 0x6d, 0x65, 0x6e, 0x74, 0x3d, 0x66, 0x6f,
                    0x6f, 0x00,
                ],
                vec![
                    ParserWord::Command("/foo"),
                    ParserWord::API(".tag=xyz"),
                    ParserWord::Attribute("=comment=foo"),
                ],
            ),
        ];
        for (input, expected_output) in test_cases.into_iter() {
            let (_, output) = parse_sentence(input.as_slice()).unwrap();
            assert_eq!(expected_output, output);
        }
    }
}
