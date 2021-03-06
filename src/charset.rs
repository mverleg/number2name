use ::std::collections::HashMap;
use ::std::fmt;
use ::std::fmt::{Formatter, Write};
use ::std::ops::Index;

use crate::decode::name2number;
use crate::encode::{number2name_u128, number2name_u16, number2name_u32, number2name_u64};
use crate::typ::N2NErr;
use crate::util::lower;
use crate::{
    name2number_i128, name2number_i16, name2number_i32, name2number_i64, name2number_u128,
    name2number_u16, name2number_u32, name2number_u64, number2name_i128, number2name_i16,
    number2name_i32, number2name_i64,
};

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
#[allow(clippy::len_without_is_empty)]
impl Charset {
    pub fn case_sensitive(data: impl AsRef<str>) -> Self {
        Charset::new(data, Case::Sensitive)
    }

    pub fn case_insensitive(data: impl AsRef<str>) -> Self {
        Charset::new(data, Case::Insensitive)
    }

    /// Panics if the input contains duplicates.
    pub fn new(data: impl AsRef<str>, case: Case) -> Self {
        match Charset::try_new(data, case) {
            Some(charset) => charset,
            None => panic!("failed to initialize charset due to duplicate data"),
        }
    }

    /// Empty if the input contains duplicates or is empty.
    pub fn try_new(data: impl AsRef<str>, case: Case) -> Option<Self> {
        let data = data.as_ref();
        if data.is_empty() {
            return None;
        }
        let mut lookup = HashMap::new();
        let mut values = Vec::with_capacity(data.len());
        for (index, character) in data.chars().enumerate() {
            let unique_repr = match case {
                Case::Sensitive => character,
                Case::Insensitive => lower(character),
            };
            if lookup.contains_key(&unique_repr) {
                return None;
            }
            lookup.insert(unique_repr, index as u64);
            values.push(character)
        }
        Some(Charset {
            values,
            lookup,
            case,
        })
    }

    /// Number of characters.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Find the numberical position of a character.
    pub fn index_of(&self, character: char) -> Option<u64> {
        let representation = if self.case == Case::Sensitive {
            character
        } else {
            lower(character)
        };
        self.lookup.get(&representation).cloned()
    }

    pub fn encode(&self, number: u64) -> String {
        number2name_u64(number, &self)
    }

    pub fn encode_u16(&self, number: u16) -> String {
        number2name_u16(number, &self)
    }

    pub fn encode_u32(&self, number: u32) -> String {
        number2name_u32(number, &self)
    }

    pub fn encode_u64(&self, number: u64) -> String {
        number2name_u64(number, &self)
    }

    pub fn encode_u128(&self, number: u128) -> String {
        number2name_u128(number, &self)
    }

    pub fn encode_i16(&self, number: i16) -> String {
        number2name_i16(number, &self)
    }

    pub fn encode_i32(&self, number: i32) -> String {
        number2name_i32(number, &self)
    }

    pub fn encode_i64(&self, number: i64) -> String {
        number2name_i64(number, &self)
    }

    pub fn encode_i128(&self, number: i128) -> String {
        number2name_i128(number, &self)
    }

    pub fn decode(&self, text: impl AsRef<str>) -> Result<u64, N2NErr> {
        name2number(text, &self)
    }

    pub fn decode_u16(&self, text: impl AsRef<str>) -> Result<u16, N2NErr> {
        name2number_u16(text, &self)
    }

    pub fn decode_u32(&self, text: impl AsRef<str>) -> Result<u32, N2NErr> {
        name2number_u32(text, &self)
    }

    pub fn decode_u64(&self, text: impl AsRef<str>) -> Result<u64, N2NErr> {
        name2number_u64(text, &self)
    }

    pub fn decode_u128(&self, text: impl AsRef<str>) -> Result<u128, N2NErr> {
        name2number_u128(text, &self)
    }

    pub fn decode_i16(&self, text: impl AsRef<str>) -> Result<i16, N2NErr> {
        name2number_i16(text, &self)
    }

    pub fn decode_i32(&self, text: impl AsRef<str>) -> Result<i32, N2NErr> {
        name2number_i32(text, &self)
    }

    pub fn decode_i64(&self, text: impl AsRef<str>) -> Result<i64, N2NErr> {
        name2number_i64(text, &self)
    }

    pub fn decode_i128(&self, text: impl AsRef<str>) -> Result<i128, N2NErr> {
        name2number_i128(text, &self)
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
            assert_eq!(charset.index_of('a').unwrap(), 0);
            assert_eq!(charset.index_of('b').unwrap(), 1);
            assert_eq!(charset.index_of('B').unwrap(), 2);
            assert_eq!(charset.index_of('A').unwrap(), 3);
            Ok(())
        }

        #[test]
        fn case_insensitive() -> Result<(), ()> {
            let charset = Charset::case_insensitive("AbCd");
            assert_eq!(charset.index_of('A').unwrap(), 0);
            assert_eq!(charset.index_of('b').unwrap(), 1);
            assert_eq!(charset.index_of('C').unwrap(), 2);
            assert_eq!(charset.index_of('d').unwrap(), 3);
            Ok(())
        }

        #[test]
        fn do_not_ignore_case() -> Result<(), ()> {
            let charset = Charset::case_sensitive("Ab");
            assert!(charset.index_of('a').is_none());
            assert!(charset.index_of('B').is_none());
            Ok(())
        }

        #[test]
        fn ignore_case_digit() -> Result<(), ()> {
            let charset = Charset::case_insensitive("0123456789");
            assert_eq!(charset.index_of('0').unwrap(), 0);
            assert_eq!(charset.index_of('1').unwrap(), 1);
            assert_eq!(charset.index_of('2').unwrap(), 2);
            assert_eq!(charset.index_of('3').unwrap(), 3);
            assert_eq!(charset.index_of('4').unwrap(), 4);
            assert_eq!(charset.index_of('5').unwrap(), 5);
            assert_eq!(charset.index_of('6').unwrap(), 6);
            assert_eq!(charset.index_of('7').unwrap(), 7);
            assert_eq!(charset.index_of('8').unwrap(), 8);
            assert_eq!(charset.index_of('9').unwrap(), 9);
            Ok(())
        }

        #[test]
        fn ignore_case_single() -> Result<(), ()> {
            let charset = Charset::case_insensitive("A");
            assert_eq!(charset.index_of('a').unwrap(), 0);
            Ok(())
        }

        #[test]
        fn ignore_case_long() -> Result<(), ()> {
            let charset = Charset::case_insensitive("AbCdEfGhIjKlMnOp");
            assert_eq!(charset.index_of('a').unwrap(), 0);
            assert_eq!(charset.index_of('B').unwrap(), 1);
            assert_eq!(charset.index_of('c').unwrap(), 2);
            assert_eq!(charset.index_of('D').unwrap(), 3);
            assert_eq!(charset.index_of('e').unwrap(), 4);
            assert_eq!(charset.index_of('F').unwrap(), 5);
            assert_eq!(charset.index_of('g').unwrap(), 6);
            assert_eq!(charset.index_of('H').unwrap(), 7);
            assert_eq!(charset.index_of('i').unwrap(), 8);
            assert_eq!(charset.index_of('J').unwrap(), 9);
            assert_eq!(charset.index_of('k').unwrap(), 10);
            assert_eq!(charset.index_of('L').unwrap(), 11);
            assert_eq!(charset.index_of('m').unwrap(), 12);
            assert_eq!(charset.index_of('N').unwrap(), 13);
            assert_eq!(charset.index_of('o').unwrap(), 14);
            assert_eq!(charset.index_of('P').unwrap(), 15);
            Ok(())
        }
    }
}
