use crate::{
    signed2unsigned_128, signed2unsigned_16, signed2unsigned_32, signed2unsigned_64, Charset,
};

macro_rules! number2name_for_type {
    ($name: ident, $int:ty) => {
        /// Convert a number to a short string representation using the given character set.
        pub fn $name(number: impl Into<$int>, charset: &Charset) -> String {
            let size = charset.len() as $int;
            let mut remainder = number.into();
            let mut name = Vec::new();
            loop {
                let index = remainder % size;
                name.push(index as usize);
                remainder /= size;
                if remainder == 0 {
                    break;
                }
                remainder -= 1;
            }
            name.into_iter().map(|index| charset[index]).rev().collect()
        }
    };
}

number2name_for_type!(number2name_u16, u16);
number2name_for_type!(number2name_u32, u32);
number2name_for_type!(number2name_u64, u64);
number2name_for_type!(number2name_u128, u128);

pub fn number2name_i16(number: impl Into<i16>, charset: &Charset) -> String {
    number2name_u16(signed2unsigned_16(number.into()), charset)
}

pub fn number2name_i32(number: impl Into<i32>, charset: &Charset) -> String {
    number2name_u32(signed2unsigned_32(number.into()), charset)
}

pub fn number2name_i64(number: impl Into<i64>, charset: &Charset) -> String {
    number2name_u64(signed2unsigned_64(number.into()), charset)
}

pub fn number2name_i128(number: impl Into<i128>, charset: &Charset) -> String {
    number2name_u128(signed2unsigned_128(number.into()), charset)
}

/// Convert a number to a short string representation using the given character set.
pub fn number2name(number: impl Into<u64>, charset: &Charset) -> String {
    // compiler, please inline this!
    number2name_u64(number, charset)
}

#[cfg(test)]
mod general {
    use super::*;
    use crate::Charset;

    #[test]
    fn single_char_first() {
        let charset = Charset::case_sensitive("aBcD");
        let text = number2name(0u64, &charset);
        assert_eq!(text, "a");
    }

    #[test]
    fn single_char_last() {
        let charset = Charset::case_sensitive("aBcD");
        let text = number2name(4u64 - 1, &charset);
        assert_eq!(text, "D");
    }

    #[test]
    fn two_char_first() {
        let charset = Charset::case_sensitive("aBcD");
        let text = number2name(4u64, &charset);
        assert_eq!(text, "aa");
    }

    #[test]
    fn two_char_last() {
        let charset = Charset::case_sensitive("aBcD");
        let text = number2name(4u64 + 16 - 1, &charset);
        assert_eq!(text, "DD");
    }

    #[test]
    fn three_char_first() {
        let charset = Charset::case_sensitive("aBcD");
        let text = number2name(4u64 + 16, &charset);
        assert_eq!(text, "aaa");
    }

    #[test]
    fn three_char_last() {
        let charset = Charset::case_sensitive("aBcD");
        let text = number2name(4u64 + 16 + 64 - 1, &charset);
        assert_eq!(text, "DDD");
    }

    #[test]
    fn four_char_first() {
        let charset = Charset::case_sensitive("aBcD");
        let text = number2name(4u64 + 16 + 64, &charset);
        assert_eq!(text, "aaaa");
    }

    #[test]
    fn four_char_last() {
        let charset = Charset::case_sensitive("aBcD");
        let text = number2name(4u64 + 16 + 64 + 256 - 1, &charset);
        assert_eq!(text, "DDDD");
    }

    #[test]
    fn random_value() {
        let charset = Charset::case_sensitive("aBcD");
        let text = number2name(744u64, &charset);
        assert_eq!(text, "BcBBa");
    }

    #[test]
    fn large_charset() {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let text = number2name(744u64, &charset);
        assert_eq!(text, "aBq");
    }

    #[test]
    fn small_charset() {
        let charset = Charset::case_sensitive("aB");
        let text = number2name(744u64, &charset);
        assert_eq!(text, "aBBBaBaBa");
    }

    #[test]
    fn tiny_charset() {
        let charset = Charset::case_sensitive("0");
        let text = number2name(7u64, &charset);
        assert_eq!(text, "00000000");
    }
}

#[cfg(test)]
mod type_u16 {
    use super::*;
    use crate::Charset;

    #[test]
    fn unsigned() {
        let charset = Charset::case_sensitive("aBcD");
        let text = number2name_u16(4u16 + 16, &charset);
        assert_eq!(text, "aaa");
    }

    #[test]
    fn signed() {
        let charset = Charset::case_sensitive("aBcD");
        let text = number2name_i16(-13i16, &charset);
        assert_eq!(text, "aBB");
    }

    #[test]
    fn unsigned_maximum() {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let text = number2name_u16(std::u16::MAX, &charset);
        assert_eq!(text, "cRXP");
    }

    #[test]
    fn signed_maximum() {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let text = number2name_i16(std::i16::MAX, &charset);
        assert_eq!(text, "cRXo");
    }

    #[test]
    fn signed_minimum() {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let text = number2name_i16(std::i16::MIN, &charset);
        assert_eq!(text, "cRXP");
    }
}

#[cfg(test)]
mod type_u32 {
    use super::*;
    use crate::Charset;

    #[test]
    fn unsigned() {
        let charset = Charset::case_sensitive("aBcD");
        let text = number2name_u32(4u32 + 16, &charset);
        assert_eq!(text, "aaa");
    }

    #[test]
    fn signed() {
        let charset = Charset::case_sensitive("aBcD");
        let text = number2name_i32(-13i32, &charset);
        assert_eq!(text, "aBB");
    }

    #[test]
    fn unsigned_maximum() {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let text = number2name_u32(std::u32::MAX, &charset);
        assert_eq!(text, "mwLqkwV");
    }

    #[test]
    fn signed_maximum() {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let text = number2name_i32(std::i32::MAX, &charset);
        assert_eq!(text, "mwLqkwu");
    }

    #[test]
    fn signed_minimum() {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let text = number2name_i32(std::i32::MIN, &charset);
        assert_eq!(text, "mwLqkwV");
    }
}

#[cfg(test)]
mod type_u64 {
    use super::*;
    use crate::Charset;

    #[test]
    fn unsigned() {
        let charset = Charset::case_sensitive("aBcD");
        let text = number2name_u64(4u64 + 16, &charset);
        assert_eq!(text, "aaa");
    }

    #[test]
    fn signed() {
        let charset = Charset::case_sensitive("aBcD");
        let text = number2name_i64(-13i64, &charset);
        assert_eq!(text, "aBB");
    }

    #[test]
    fn unsigned_maximum() {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let text = number2name_u64(std::u64::MAX, &charset);
        assert_eq!(text, "gkgwByLwRXTLPP");
    }

    #[test]
    fn signed_maximum() {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let text = number2name_i64(std::i64::MAX, &charset);
        assert_eq!(text, "gkgwByLwRXTLPo");
    }

    #[test]
    fn signed_minimum() {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let text = number2name_i64(std::i64::MIN, &charset);
        assert_eq!(text, "gkgwByLwRXTLPP");
    }
}

#[cfg(test)]
mod type_u128 {
    use super::*;
    use crate::Charset;

    #[test]
    fn unsigned() {
        let charset = Charset::case_sensitive("aBcD");
        let text = number2name_u128(4u128 + 16u128, &charset);
        assert_eq!(text, "aaa");
    }

    #[test]
    fn signed() {
        let charset = Charset::case_sensitive("aBcD");
        let text = number2name_i128(-13i128, &charset);
        assert_eq!(text, "aBB");
    }

    #[test]
    fn unsigned_maximum() {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let text = number2name_u128(std::u128::MAX, &charset);
        assert_eq!(text, "BcgDeNLqRqwDsLRugsNLBTmFiJaV");
    }

    #[test]
    fn signed_maximum() {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let text = number2name_i128(std::i128::MAX, &charset);
        assert_eq!(text, "BcgDeNLqRqwDsLRugsNLBTmFiJau");
    }

    #[test]
    fn signed_minimum() {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let text = number2name_i128(std::i128::MIN, &charset);
        assert_eq!(text, "BcgDeNLqRqwDsLRugsNLBTmFiJaV");
    }
}
