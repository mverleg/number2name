use crate::typ::N2NErr;
use crate::{Charset, unsigned2signed_16, unsigned2signed_32, unsigned2signed_64, unsigned2signed_128};

macro_rules! name2number_for_type {
    ($name: ident, $int:ty) => {
        /// Convert a string encoded using the given charset back to the number it represents.
        pub fn $name(text: impl AsRef<str>, charset: &Charset) -> Result<$int, N2NErr> {
            fn get_index(character: char, charset: &Charset) -> Result<u64, N2NErr> {
                match charset.index_of(character) {
                    Some(i) => Ok(i as u64),
                    None => Err(N2NErr::InvalidCharacter {
                        character,
                        charset: charset.clone(),
                    }),
                }
            }

            let text = text.as_ref();
            let size = charset.len() as $int;
            // Handle the first letter separately
            let mut number: $int = if let Some(first_char) = text.chars().rev().next() {
                get_index(first_char, charset)? as $int
            } else {
                return Err(N2NErr::EmptyInput);
            };
            // Handle the other letters, with special case for near-overflow
            let mut scale: $int = 1;
            for character in text.chars().rev().skip(1) {
                let value = get_index(character, charset)? as $int;
                match scale.checked_mul(size) {
                    Some(new_scale) => {
                        scale = new_scale;
                        number = match number.checked_add((value + 1) * scale) {
                            Some(n) => n,
                            None => {
                                return Err(N2NErr::TooLarge {
                                    charset: charset.clone(),
                                })
                            }
                        }
                    }
                    None => {
                        return Err(N2NErr::TooLarge {
                            charset: charset.clone(),
                        })
                    }
                };
            }
            Ok(number)
        }
    }
}

name2number_for_type!(name2number_u16, u16);
name2number_for_type!(name2number_u32, u32);
name2number_for_type!(name2number_u64, u64);
name2number_for_type!(name2number_u128, u128);

pub fn name2number_i16(text: impl AsRef<str>, charset: &Charset) -> Result<i16, N2NErr> {
    Ok(unsigned2signed_16(name2number_u16(text.as_ref(), charset)?))
}

pub fn name2number_i32(text: impl AsRef<str>, charset: &Charset) -> Result<i32, N2NErr> {
    Ok(unsigned2signed_32(name2number_u32(text.as_ref(), charset)?))
}

pub fn name2number_i64(text: impl AsRef<str>, charset: &Charset) -> Result<i64, N2NErr> {
    Ok(unsigned2signed_64(name2number_u64(text.as_ref(), charset)?))
}

pub fn name2number_i128(text: impl AsRef<str>, charset: &Charset) -> Result<i128, N2NErr> {
    Ok(unsigned2signed_128(name2number_u128(text.as_ref(), charset)?))
}

/// Convert a string encoded using the given charset back to the number it represents.
pub fn name2number(text: impl AsRef<str>, charset: &Charset) -> Result<u64, N2NErr> {
    name2number_u64(text, charset)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Charset;

    #[test]
    fn single_char_first() -> Result<(), N2NErr> {
        let charset = Charset::case_sensitive("aBcD");
        let nr = name2number("a", &charset)?;
        assert_eq!(nr, 0);
        Ok(())
    }

    #[test]
    fn single_char_last() -> Result<(), N2NErr> {
        let charset = Charset::case_sensitive("aBcD");
        let nr = name2number("D", &charset)?;
        assert_eq!(nr, 4 - 1);
        Ok(())
    }

    #[test]
    fn two_char_first() -> Result<(), N2NErr> {
        let charset = Charset::case_sensitive("aBcD");
        let nr = name2number("aa", &charset)?;
        assert_eq!(nr, 4);
        Ok(())
    }

    #[test]
    fn two_char_last() -> Result<(), N2NErr> {
        let charset = Charset::case_sensitive("aBcD");
        let nr = name2number("DD", &charset)?;
        assert_eq!(nr, 4 + 16 - 1);
        Ok(())
    }

    #[test]
    fn three_char_first() -> Result<(), N2NErr> {
        let charset = Charset::case_sensitive("aBcD");
        let nr = name2number("aaa", &charset)?;
        assert_eq!(nr, 4 + 16);
        Ok(())
    }

    #[test]
    fn three_char_last() -> Result<(), N2NErr> {
        let charset = Charset::case_sensitive("aBcD");
        let nr = name2number("DDD", &charset)?;
        assert_eq!(nr, 4 + 16 + 64 - 1);
        Ok(())
    }

    #[test]
    fn four_char_first() -> Result<(), N2NErr> {
        let charset = Charset::case_sensitive("aBcD");
        let nr = name2number("aaaa", &charset)?;
        assert_eq!(nr, 4 + 16 + 64);
        Ok(())
    }

    #[test]
    fn four_char_last() -> Result<(), N2NErr> {
        let charset = Charset::case_sensitive("aBcD");
        let nr = name2number("DDDD", &charset)?;
        assert_eq!(nr, 4 + 16 + 64 + 256 - 1);
        Ok(())
    }

    #[test]
    fn random_value() -> Result<(), N2NErr> {
        let charset = Charset::case_sensitive("aBcD");
        let nr = name2number("BcBBa", &charset)?;
        assert_eq!(nr, 744);
        Ok(())
    }

    #[test]
    fn large_charset() -> Result<(), N2NErr> {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let nr = name2number("aBq", &charset)?;
        assert_eq!(nr, 744);
        Ok(())
    }

    #[test]
    fn small_charset() -> Result<(), N2NErr> {
        let charset = Charset::case_sensitive("aB");
        let nr = name2number("aBBBaBaBa", &charset)?;
        assert_eq!(nr, 744);
        Ok(())
    }

    #[test]
    fn tiny_charset() -> Result<(), N2NErr> {
        let charset = Charset::case_sensitive("0");
        let nr = name2number("00000000", &charset)?;
        assert_eq!(nr, 7);
        Ok(())
    }

    #[test]
    fn valid_case_insensitive_single() -> Result<(), N2NErr> {
        let charset = Charset::case_insensitive("0a");
        let nr = name2number("A", &charset)?;
        assert_eq!(nr, 1);
        Ok(())
    }

    #[test]
    fn valid_case_insensitive_double() -> Result<(), N2NErr> {
        let charset = Charset::case_insensitive("aBcD");
        let nr = name2number("bd", &charset)?;
        assert_eq!(nr, 8 + 3);
        Ok(())
    }

    #[test]
    fn invalid_case_insensitive_single() -> Result<(), N2NErr> {
        let charset = Charset::case_insensitive("aBcD");
        match name2number("e", &charset).unwrap_err() {
            N2NErr::InvalidCharacter {
                character,
                charset: _,
            } => assert_eq!(character, 'e'),
            _ => panic!("wrong error"),
        }
        Ok(())
    }

    #[test]
    fn invalid_case_insensitive_long() -> Result<(), N2NErr> {
        let charset = Charset::case_insensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        match name2number("gkgwByLwRXT7Pq", &charset).unwrap_err() {
            N2NErr::InvalidCharacter {
                character,
                charset: _,
            } => assert_eq!(character, '7'),
            _ => panic!("wrong error"),
        }
        Ok(())
    }

    #[test]
    fn invalid_case_sensitive_single() -> Result<(), N2NErr> {
        let charset = Charset::case_sensitive("aBcD");
        match name2number("b", &charset).unwrap_err() {
            N2NErr::InvalidCharacter {
                character,
                charset: _,
            } => assert_eq!(character, 'b'),
            _ => panic!("wrong error"),
        }
        Ok(())
    }

    #[test]
    fn invalid_case_sensitive_long() -> Result<(), N2NErr> {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        match name2number("gkgwByLwRXTlPP", &charset).unwrap_err() {
            N2NErr::InvalidCharacter {
                character,
                charset: _,
            } => assert_eq!(character, 'l'),
            _ => panic!("wrong error"),
        }
        Ok(())
    }

    #[test]
    fn below_overflow() -> Result<(), N2NErr> {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let nr = name2number("gkgwByLwRXTLPo", &charset)?;
        assert_eq!(nr, std::u64::MAX - 1);
        Ok(())
    }

    #[test]
    fn too_long() -> Result<(), N2NErr> {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        match name2number("aaaaaaaaaaaaaaa", &charset).unwrap_err() {
            N2NErr::TooLarge { charset: _ } => {}
            _ => panic!("wrong error"),
        }
        Ok(())
    }

    #[test]
    fn limit_plus_one() -> Result<(), N2NErr> {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        match name2number("gkgwByLwRXTLPq", &charset).unwrap_err() {
            N2NErr::TooLarge { charset: _ } => {}
            _ => panic!("wrong error"),
        }
        Ok(())
    }

    #[test]
    fn empty_input() -> Result<(), N2NErr> {
        let charset = Charset::case_sensitive("aBcD");
        match name2number("", &charset).unwrap_err() {
            N2NErr::EmptyInput => {}
            _ => panic!("wrong error"),
        }
        Ok(())
    }
}

#[cfg(test)]
mod type_u16 {
    use super::*;
    use crate::Charset;

    #[test]
    fn unsigned() {
        let charset = Charset::case_sensitive("aBcD");
        let nr = name2number_u16("aaa", &charset).unwrap();
        assert_eq!(nr, 4u16 + 16u16);
    }

    #[test]
    fn signed() {
        let charset = Charset::case_sensitive("aBcD");
        let nr = name2number_i16("aBB", &charset).unwrap();
        assert_eq!(nr, -13i16);
    }

    #[test]
    fn unsigned_maximum() {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let nr = name2number_u16("cRXP", &charset).unwrap();
        assert_eq!(nr, ::std::u16::MAX);
    }

    #[test]
    fn signed_maximum() {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let nr = name2number_i16("cRXo", &charset).unwrap();
        assert_eq!(nr, ::std::i16::MAX);
    }

    #[test]
    fn signed_minimum() {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let nr = name2number_i16("cRXP", &charset).unwrap();
        assert_eq!(nr, ::std::i16::MIN);
    }
}

#[cfg(test)]
mod type_u32 {
    use super::*;
    use crate::Charset;

    #[test]
    fn unsigned() {
        let charset = Charset::case_sensitive("aBcD");
        let nr = name2number_u32("aaa", &charset).unwrap();
        assert_eq!(nr, 4u32 + 16u32);
    }

    #[test]
    fn signed() {
        let charset = Charset::case_sensitive("aBcD");
        let nr = name2number_i32("aBB", &charset).unwrap();
        assert_eq!(nr, -13i32);
    }

    #[test]
    fn unsigned_maximum() {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let nr = name2number_u32("mwLqkwV", &charset).unwrap();
        assert_eq!(nr, ::std::u32::MAX);
    }

    #[test]
    fn signed_maximum() {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let nr = name2number_i32("mwLqkwu", &charset).unwrap();
        assert_eq!(nr, ::std::i32::MAX);
    }

    #[test]
    fn signed_minimum() {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let nr = name2number_i32("mwLqkwV", &charset).unwrap();
        assert_eq!(nr, ::std::i32::MIN);
    }
}

#[cfg(test)]
mod type_u64 {
    use super::*;
    use crate::Charset;

    #[test]
    fn unsigned() {
        let charset = Charset::case_sensitive("aBcD");
        let nr = name2number_u64("aaa", &charset).unwrap();
        assert_eq!(nr, 4u64 + 16u64);
    }

    #[test]
    fn signed() {
        let charset = Charset::case_sensitive("aBcD");
        let nr = name2number_i64("aBB", &charset).unwrap();
        assert_eq!(nr, -13i64);
    }

    #[test]
    fn unsigned_maximum() {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let nr = name2number_u64("gkgwByLwRXTLPP", &charset).unwrap();
        assert_eq!(nr, ::std::u64::MAX);
    }

    #[test]
    fn signed_maximum() {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let nr = name2number_i64("gkgwByLwRXTLPo", &charset).unwrap();
        assert_eq!(nr, ::std::i64::MAX);
    }

    #[test]
    fn signed_minimum() {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let nr = name2number_i64("gkgwByLwRXTLPP", &charset).unwrap();
        assert_eq!(nr, ::std::i64::MIN);
    }
}

#[cfg(test)]
mod type_u128 {
    use super::*;
    use crate::Charset;

    #[test]
    fn unsigned() {
        let charset = Charset::case_sensitive("aBcD");
        let nr = name2number_u128("aaa", &charset).unwrap();
        assert_eq!(nr, 4u128 + 16u128);
    }

    #[test]
    fn signed() {
        let charset = Charset::case_sensitive("aBcD");
        let nr = name2number_i128("aBB", &charset).unwrap();
        assert_eq!(nr, -13i128);
    }

    #[test]
    fn unsigned_maximum() {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let nr = name2number_u128("BcgDeNLqRqwDsLRugsNLBTmFiJaV", &charset).unwrap();
        assert_eq!(nr, ::std::u128::MAX);
    }

    #[test]
    fn signed_maximum() {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let nr = name2number_i128("BcgDeNLqRqwDsLRugsNLBTmFiJau", &charset).unwrap();
        assert_eq!(nr, ::std::i128::MAX);
    }

    #[test]
    fn signed_minimum() {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let nr = name2number_i128("BcgDeNLqRqwDsLRugsNLBTmFiJaV", &charset).unwrap();
        assert_eq!(nr, ::std::i128::MIN);
    }
}
