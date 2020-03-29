use crate::Charset;
use crate::typ::N2NErr;

pub fn name2number<'a>(text: impl Into<&'a str>, charset: &Charset) -> Result<u64, N2NErr> {
    let mut text = text.into();
    let size = charset.len() as u64;
    let mut number = 0;
    let mut scale = 1;
    for character in text.chars().rev() {
        dbg!(character);  //TODO @mverleg: remove
        let value = match charset.index_of(character) {
            Ok(i) => i,
            Err(()) => return Err(N2NErr::InvalidCharacter { character, charset: charset.clone() }),
        };
        number += (value + 1) * scale;
        dbg!(number);  //TODO @mverleg: remove
        scale *= size;
    }


    Ok(number - 1)

//    let size = charset.len() as u64;
//     let mut remainder = number.into();
//     let mut name = Vec::new();
//     loop {
//         let index = remainder % size;
//         name.push(index as usize);
//         remainder /= size;
//         if remainder == 0 {
//             break;
//         }
//         remainder -= 1;
//     }
//     name.into_iter()
//         .map(|index| charset[index])
//         .rev()
//         .collect()
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

    //TODO @mark: more tests for invalid input
    //TODO @mark: more tests for length

    #[test]
    fn near_overflow() {
        let charset = Charset::case_sensitive("aBcDeFgHiJkLmNoPqRsTuVwXyZ");
        let nr = name2number("gkgwByLwRXTLPP", &charset).unwrap();
        assert_eq!(nr, std::u64::MAX);
    }
}
