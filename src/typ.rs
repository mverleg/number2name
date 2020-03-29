use crate::Charset;

#[derive(Debug, Clone)]
pub enum N2NErr {
    EmptyInput,
    TooLarge { charset: Charset },
    InvalidCharacter { character: char, charset: Charset },
}

impl Into<String> for N2NErr {
    fn into(self) -> String {
        match self {
            N2NErr::EmptyInput => "input was empty while decoding".to_owned(),
            N2NErr::TooLarge { charset } => format!(
                "input was too long to decode number; maximum: {}", charset.encode(std::u64::MAX)),
            N2NErr::InvalidCharacter { character, charset } => format!(
                "encountered invalid character '{}' while decoding to number; allowed: '{}'",
                character, charset),
        }
    }
}
