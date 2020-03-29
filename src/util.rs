
/// Return the lowercase representation character.
/// Fails if the lowercase representation is not a single character.
pub fn lower(character: char) -> char {
    let mut lc = character.to_lowercase();
    assert!(lc.len() > 0);
    if lc.len() > 1 {
        panic!("character {} is not allowed because its lower-case has more \
        than one unicode code point ({})", character, lc);
    }
    lc.next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_lowercase() {
        assert_eq!(lower('A'), 'a');
        assert_eq!(lower('a'), 'a');
    }

    #[test]
    fn test_no_case() {
        assert_eq!(lower('8'), '8');
        assert_eq!(lower('+'), '+');
        assert_eq!(lower('中'), '中');
    }

    #[test]
    fn test_invalid_lowercase() {
        unimplemented!()
    }
}