use crate::Charset;

pub fn number2name(number: impl Into<u64>, charset: &Charset) -> String {
    let size = charset.len() as u64;
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
    name.into_iter()
        .map(|index| charset[index])
        .rev()
        .collect()
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn near_overflow() {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let text = number2name(std::u64::MAX, &charset);
        assert_eq!(text, "gkgwByLwRXTLPP");
    }
}
