use crate::Charset;
use crate::typ::N2NErr;

pub fn name2number<'a>(text: impl Into<&'a str>, charset: &Charset) -> Result<u64, N2NErr> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Charset;

    #[test]
    fn single_char_first() {
        let charset = Charset::case_sensitive("aBcD");
        let nr = name2number("a", &charset).unwrap();
        assert_eq!(nr, 0);
    }

    #[test]
    fn single_char_last() {
        let charset = Charset::case_sensitive("aBcD");
        let nr = name2number("D", &charset).unwrap();
        assert_eq!(nr, 4 - 1);
    }

    #[test]
    fn two_char_first() {
        let charset = Charset::case_sensitive("aBcD");
        let nr = name2number("aa", &charset).unwrap();
        assert_eq!(nr, 4);
    }

    #[test]
    fn two_char_last() {
        let charset = Charset::case_sensitive("aBcD");
        let nr = name2number("DD", &charset).unwrap();
        assert_eq!(nr, 4 + 16 - 1);
    }

    #[test]
    fn three_char_first() {
        let charset = Charset::case_sensitive("aBcD");
        let nr = name2number("aaa", &charset).unwrap();
        assert_eq!(nr, 4 + 16);
    }

    #[test]
    fn three_char_last() {
        let charset = Charset::case_sensitive("aBcD");
        let nr = name2number("DDD", &charset).unwrap();
        assert_eq!(nr, 4 + 16 + 64 - 1);
    }

    #[test]
    fn four_char_first() {
        let charset = Charset::case_sensitive("aBcD");
        let nr = name2number("aaaa", &charset).unwrap();
        assert_eq!(nr, 4 + 16 + 64);
    }

    #[test]
    fn four_char_last() {
        let charset = Charset::case_sensitive("aBcD");
        let nr = name2number("DDDD", &charset).unwrap();
        assert_eq!(nr, 4 + 16 + 64 + 256 - 1);
    }

    #[test]
    fn random_value() {
        let charset = Charset::case_sensitive("aBcD");
        let nr = name2number("BcBBa", &charset).unwrap();
        assert_eq!(nr, 744);
    }

    #[test]
    fn large_charset() {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let nr = name2number("aBq", &charset).unwrap();
        assert_eq!(nr, 744);
    }

    #[test]
    fn small_charset() {
        let charset = Charset::case_sensitive("aB");
        let nr = name2number("aBBBaBaBa", &charset).unwrap();
        assert_eq!(nr, 744);
    }

    #[test]
    fn tiny_charset() {
        let charset = Charset::case_sensitive("0");
        let nr = name2number("00000000", &charset).unwrap();
        assert_eq!(nr, 7);
    }

    #[test]
    fn near_overflow() {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let nr = name2number("gkgwByLwRXTLPP", &charset).unwrap();
        assert_eq!(nr, std::u64::MAX);
    }
}
