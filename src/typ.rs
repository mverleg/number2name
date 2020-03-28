use crate::Charset;

#[derive(Debug, Clone)]
pub enum N2NErr {
    TooLong { max_length: usize },
    InvalidCharacter { character: char, charset: Charset },
}

impl Into<String> for N2NErr {
    fn into(self) -> String {
        match self {
            N2NErr::TooLong { max_length } => format!(
                "input was too long to decode number; maximum: {}", max_length),
            N2NErr::InvalidCharacter { character, charset } => format!(
                "encountered invalid character '{}' while decoding to number; allowed: '{}'",
                character, charset),
        }
    }
}
