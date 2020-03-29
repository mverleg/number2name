use ::std::ops::Index;
use ::std::collections::HashSet;
use ::std::fmt;
use std::fmt::{Formatter, Write};
use crate::encode::number2name;
use crate::decode::name2number;
use crate::typ::N2NErr;
use crate::util::lower;

#[derive(Debug, Clone, Copy)]
pub enum Case {
    Sensitive,
    Insensitive,
}

#[derive(Clone)]
pub struct Charset {
    values: Vec<char>,
    case: Case,
}

impl fmt::Debug for Charset {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("(")?;
        f.write_str(match self.case {
            Case::Sensitive => "case-sensitive",
            Case::Insensitive => "case-insensitive",
        })?;
        f.write_str(" character set: [")?;
        for character in &self.values {
            f.write_char(*character)?;
        }
        f.write_str("])")
    }
}

impl fmt::Display for Charset {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for character in &self.values {
            f.write_char(*character)?;
        }
        Ok(())
    }
}

/// A character set of unique characters in a specific order.
/// If case-insensitive, characters must have a single-character lower-case version (can be the same as upper-case).
impl Charset {
    pub fn case_sensitive<'a>(data: impl Into<&'a str>) -> Self {
        Charset::new(data, Case::Sensitive)
    }

    pub fn case_insensitive<'a>(data: impl Into<&'a str>) -> Self {
        Charset::new(data, Case::Insensitive)
    }

    /// Panics if the input contains duplicates.
    pub fn new<'a>(data: impl Into<&'a str>, case: Case) -> Self {
        match Charset::try_new(data, case) {
            Some(charset) => charset,
            None => panic!("failed to initialize charset due to duplicate data"),
        }
    }

    /// Empty if the input contains duplicates.
    pub fn try_new<'a>(data: impl Into<&'a str>, case: Case) -> Option<Self> {
        let data = data.into();
        assert!(data.len() > 0);
        let mut seen = HashSet::new();
        let mut values = Vec::with_capacity(data.len());
        for character in data.chars() {
            let unique_repr = match case {
                Case::Sensitive => character,
                Case::Insensitive => lower(character),
            };
            if seen.contains(&unique_repr) {
                return None
            }
            seen.insert(unique_repr);
            values.push(character)
        }
        Some(Charset {
            values, case
        })
    }

    /// Number of characters.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Find the numberical position of a character.
    //TODO @mark: tests
    pub fn index_of(&self, character: char) -> Result<u64, ()> {
        let representation = lower(character);
        for i in 0..self.values.len() {
            if self.values[i] == representation {
                return Ok(i as u64);
            }
        }
        Err(())
    }

    pub fn encode(&self, number: u64) -> String {
        number2name(number, &self)
    }

    pub fn decode<'a>(&self, text: impl Into<&'a str>) -> Result<u64, N2NErr> {
        name2number(text, &self)
    }
}

impl Index<usize> for Charset {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_charset() {
        let charset = Charset::try_new("Abc", Case::Insensitive);
        assert!(charset.is_some());
        let charset = charset.unwrap();
        assert_eq!(charset.len(), 3);
        assert_eq!(charset[0], 'A');
        assert_eq!(charset[1], 'b');
        assert_eq!(charset[2], 'c');
    }

    #[test]
    fn valid_case_duplicate() {
        let charset = Charset::try_new("abBA", Case::Sensitive);
        assert!(charset.is_some());
        let charset = charset.unwrap();
        assert_eq!(charset.len(), 4);
        assert_eq!(charset[0], 'a');
        assert_eq!(charset[1], 'b');
        assert_eq!(charset[2], 'B');
        assert_eq!(charset[3], 'A');
    }

    #[test]
    fn invalid_duplicate() {
        let charset = Charset::try_new("aba", Case::Insensitive);
        assert!(charset.is_none());
    }

    #[test]
    fn invalid_case_insensitive_duplicate() {
        let charset = Charset::try_new("abBA", Case::Insensitive);
        assert!(charset.is_none());
    }
}
