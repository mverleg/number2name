use crate::Charset;
use crate::typ::N2NErr;

/// Convert a string encoded using the given charset back to the number it represents.
pub fn name2number<'a>(text: impl AsRef<str>, charset: &Charset) -> Result<u64, N2NErr> {
    let text = text.as_ref();
    let size = charset.len() as u64;
    // Handle the first letter separately
    let mut number = if let Some(first_char) = text.chars().rev().next() {
        get_index(first_char, charset)?
    } else {
        return Err(N2NErr::EmptyInput)
    };
    // Handle the other letters, with special case for near-overflow
    let mut scale: u64 = 1;
    for character in text.chars().rev().skip(1) {
        dbg!(character);  //TODO @mverleg: remove
        let value = get_index(character, charset)?;
        dbg!(value);  //TODO @mverleg: remove
        match scale.checked_mul(size) {
            Some(new_scale) => {
                scale = new_scale;
                let add = (value + 1) * scale;  //TODO @mark: TEMPORARY! REMOVE THIS!
                dbg!(add);  //TODO @mverleg: remove
                number = match number.checked_add((value + 1) * scale) {
                    Some(n) => n,
                    None => return Err(N2NErr::TooLarge { charset: charset.clone() }),
                }
            },
            None => return Err(N2NErr::TooLarge { charset: charset.clone() }),
        };
    }
    Ok(number)
}

fn get_index(character: char, charset: &Charset) -> Result<u64, N2NErr> {
    match charset.index_of(character) {
        Ok(i) => Ok(i),
        Err(()) => return Err(N2NErr::InvalidCharacter { character, charset: charset.clone() }),
    }
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
            N2NErr::InvalidCharacter { character, charset: _ } => assert_eq!(character, 'e'),
            _ => panic!("wrong error"),
        }
        Ok(())
    }

    #[test]
    fn invalid_case_insensitive_long() -> Result<(), N2NErr> {
        let charset = Charset::case_insensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        match name2number("gkgwByLwRXT7Pq", &charset).unwrap_err() {
            N2NErr::InvalidCharacter { character, charset: _ } => assert_eq!(character, '7'),
            _ => panic!("wrong error"),
        }
        Ok(())
    }

    #[test]
    fn invalid_case_sensitive_single() -> Result<(), N2NErr> {
        let charset = Charset::case_sensitive("aBcD");
        match name2number("b", &charset).unwrap_err() {
            N2NErr::InvalidCharacter { character, charset: _ } => assert_eq!(character, 'b'),
            _ => panic!("wrong error"),
        }
        Ok(())
    }

    #[test]
    fn invalid_case_sensitive_long() -> Result<(), N2NErr> {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        match name2number("gkgwByLwRXTlPP", &charset).unwrap_err() {
            N2NErr::InvalidCharacter { character, charset: _ } => assert_eq!(character, 'l'),
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
    fn near_overflow() -> Result<(), N2NErr> {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let nr = name2number("gkgwByLwRXTLPP", &charset)?;
        assert_eq!(nr, std::u64::MAX);
        Ok(())
    }

    #[test]
    fn too_long() -> Result<(), N2NErr> {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        match name2number("aaaaaaaaaaaaaaa", &charset).unwrap_err() {
            N2NErr::TooLarge { charset: _ } => {},
            _ => panic!("wrong error"),
        }
        Ok(())
    }

    #[test]
    fn limit_plus_one() -> Result<(), N2NErr> {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        match name2number("gkgwByLwRXTLPq", &charset).unwrap_err() {
            N2NErr::TooLarge { charset: _ } => {},
            _ => panic!("wrong error"),
        }
        Ok(())
    }

    #[test]
    fn empty_input() -> Result<(), N2NErr> {
        let charset = Charset::case_sensitive("aBcD");
        match name2number("", &charset).unwrap_err() {
            N2NErr::EmptyInput => {},
            _ => panic!("wrong error"),
        }
        Ok(())
    }
}
