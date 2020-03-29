use ::std::collections::HashMap;
use ::std::fmt;
use ::std::fmt::{Formatter, Write};
use ::std::ops::Index;

use crate::decode::name2number;
use crate::encode::number2name;
use crate::typ::N2NErr;
use crate::util::lower;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Case {
    Sensitive,
    Insensitive,
}

#[derive(Clone)]
pub struct Charset {
    values: Vec<char>,
    lookup: HashMap<char, u64>,
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
    pub fn case_sensitive<'a>(data: impl AsRef<str>) -> Self {
        Charset::new(data, Case::Sensitive)
    }

    pub fn case_insensitive<'a>(data: impl AsRef<str>) -> Self {
        Charset::new(data, Case::Insensitive)
    }

    /// Panics if the input contains duplicates.
    pub fn new<'a>(data: impl AsRef<str>, case: Case) -> Self {
        match Charset::try_new(data, case) {
            Some(charset) => charset,
            None => panic!("failed to initialize charset due to duplicate data"),
        }
    }

    /// Empty if the input contains duplicates.
    pub fn try_new<'a>(data: impl AsRef<str>, case: Case) -> Option<Self> {
        let data = data.as_ref();
        assert!(data.len() > 0);
        let mut lookup = HashMap::new();
        let mut values = Vec::with_capacity(data.len());
        for (index, character) in data.chars().enumerate() {
            let unique_repr = match case {
                Case::Sensitive => character,
                Case::Insensitive => lower(character),
            };
            if lookup.contains_key(&unique_repr) {
                return None
            }
            lookup.insert(unique_repr, index as u64);
            values.push(character)
        }
        Some(Charset {
            values, lookup, case
        })
    }

    /// Number of characters.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Find the numberical position of a character.
    pub fn index_of(&self, character: char) -> Result<u64, ()> {
        unimplemented!();  //TODO @mark: TEMPORARY! REMOVE THIS!
        if self.case == Case::Sensitive {
            for i in 0..self.values.len() {
                if self.values[i] == character {
                    return Ok(i as u64);
                }
            }
        } else {
            let representation = lower(character);
            for i in 0..self.values.len() {
                if lower(self.values[i]) == representation {
                    return Ok(i as u64);
                }
            }
        };
        Err(())
    }

    pub fn encode(&self, number: u64) -> String {
        number2name(number, &self)
    }

    pub fn decode<'a>(&self, text: impl AsRef<str>) -> Result<u64, N2NErr> {
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

    mod create {
        use super::*;

        #[test]
        fn valid_charset() {
            let charset = Charset::try_new("Abc", Case::Insensitive);
            assert!(charset.is_some());
            let charset = charset.unwrap();
            assert_eq!(charset.len(), 3);
        }

        #[test]
        fn valid_case_duplicate() {
            let charset = Charset::try_new("abBA", Case::Sensitive);
            assert!(charset.is_some());
            let charset = charset.unwrap();
            assert_eq!(charset.len(), 4);
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

        #[test]
        #[should_panic]
        fn panic_mode() {
            Charset::case_insensitive("abBA");
        }
    }

    mod indexing {
        use super::*;

        #[test]
        fn case_sensitive() {
            let charset = Charset::case_sensitive("abBA");
            assert_eq!(charset[0], 'a');
            assert_eq!(charset[1], 'b');
            assert_eq!(charset[2], 'B');
            assert_eq!(charset[3], 'A');
        }

        #[test]
        fn case_insensitive() {
            let charset = Charset::case_insensitive("AbCd");
            assert_eq!(charset[0], 'A');
            assert_eq!(charset[1], 'b');
            assert_eq!(charset[2], 'C');
            assert_eq!(charset[3], 'd');
        }

        #[test]
        #[should_panic]
        fn out_of_bounds() {
            let charset = Charset::case_insensitive("AbCd");
            assert_eq!(charset[4], 'd');
        }
    }

    mod index_of {
        use super::*;

        #[test]
        fn case_sensitive() -> Result<(), ()> {
            let charset = Charset::case_sensitive("abBA");
            assert_eq!(charset.index_of('a')?, 0);
            assert_eq!(charset.index_of('b')?, 1);
            assert_eq!(charset.index_of('B')?, 2);
            assert_eq!(charset.index_of('A')?, 3);
            Ok(())
        }

        #[test]
        fn case_insensitive() -> Result<(), ()> {
            let charset = Charset::case_insensitive("AbCd");
            assert_eq!(charset.index_of('A')?, 0);
            assert_eq!(charset.index_of('b')?, 1);
            assert_eq!(charset.index_of('C')?, 2);
            assert_eq!(charset.index_of('d')?, 3);
            Ok(())
        }

        #[test]
        fn do_not_ignore_case() -> Result<(), ()> {
            let charset = Charset::case_sensitive("Ab");
            assert!(charset.index_of('a').is_err());
            assert!(charset.index_of('B').is_err());
            Ok(())
        }

        #[test]
        fn ignore_case_digit() -> Result<(), ()> {
            let charset = Charset::case_insensitive("0123456789");
            assert_eq!(charset.index_of('0')?, 0);
            assert_eq!(charset.index_of('1')?, 1);
            assert_eq!(charset.index_of('2')?, 2);
            assert_eq!(charset.index_of('3')?, 3);
            assert_eq!(charset.index_of('4')?, 4);
            assert_eq!(charset.index_of('5')?, 5);
            assert_eq!(charset.index_of('6')?, 6);
            assert_eq!(charset.index_of('7')?, 7);
            assert_eq!(charset.index_of('8')?, 8);
            assert_eq!(charset.index_of('9')?, 9);
            Ok(())
        }

        #[test]
        fn ignore_case_single() -> Result<(), ()> {
            let charset = Charset::case_insensitive("A");
            assert_eq!(charset.index_of('a')?, 0);
            Ok(())
        }

        #[test]
        fn ignore_case_long() -> Result<(), ()> {
            let charset = Charset::case_insensitive("AbCdEfGhIjKlMnOp");
            assert_eq!(charset.index_of('a')?, 0);
            assert_eq!(charset.index_of('B')?, 1);
            assert_eq!(charset.index_of('c')?, 2);
            assert_eq!(charset.index_of('D')?, 3);
            assert_eq!(charset.index_of('e')?, 4);
            assert_eq!(charset.index_of('F')?, 5);
            assert_eq!(charset.index_of('g')?, 6);
            assert_eq!(charset.index_of('H')?, 7);
            assert_eq!(charset.index_of('i')?, 8);
            assert_eq!(charset.index_of('J')?, 9);
            assert_eq!(charset.index_of('k')?, 10);
            assert_eq!(charset.index_of('L')?, 11);
            assert_eq!(charset.index_of('m')?, 12);
            assert_eq!(charset.index_of('N')?, 13);
            assert_eq!(charset.index_of('o')?, 14);
            assert_eq!(charset.index_of('P')?, 15);
            Ok(())
        }
    }
}
