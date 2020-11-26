

/// Map a signed integer to an unsigned one, in a way that
/// * is bijective (reversible).
/// * preserves absolute value order.
/// * Puts -x right before +x (necessary because there is one more negative number).
/// Example:
/// -3 -> 5
/// -2 -> 3
/// -1 -> 1
///  0 ->  0
/// +1 -> 2
/// +2 -> 4
/// +3 -> 6
/// +4 -> 8
pub fn signed_to_unsigned(number: i64) -> u64 {
    if number >= 0 {
        2 * (number as u64)
    } else {
        // This double +1 is a bit convoluted but is necessary for the minimum value,
        // because the minimum value overflows when negated.
        (-(number + 1) as u64) * 2 + 1
    }
}

#[cfg(test)]
mod type_u64 {
    use super::*;

    #[test]
    fn zero() {
        assert_eq!(signed_to_unsigned(0), 0);
    }

    #[test]
    fn positive() {
        assert_eq!(signed_to_unsigned(1), 2);
        assert_eq!(signed_to_unsigned(15), 30);
        assert_eq!(signed_to_unsigned(1234), 2468);
    }

    #[test]
    fn negative() {
        assert_eq!(signed_to_unsigned(-1), 1);
        assert_eq!(signed_to_unsigned(-15), 29);
        assert_eq!(signed_to_unsigned(-1234), 2467);
    }

    #[test]
    fn maximum() {
        assert_eq!(signed_to_unsigned(::std::i64::MAX), ::std::u64::MAX - 1);
    }

    #[test]
    fn minimum() {
        assert_eq!(signed_to_unsigned(::std::i64::MIN), ::std::u64::MAX);
    }
}
