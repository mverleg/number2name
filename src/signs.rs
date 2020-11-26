
macro_rules! signed2unsigned_for_type {
    ($name: ident, $src: ty, $trgt: ty, $test_name: ident) => {
        pub fn $name(number: $src) -> $trgt {
            if number >= 0 {
                2 * (number as $trgt)
            } else {
                // This double +1 is a bit convoluted but is necessary for the minimum value,
                // because the minimum value overflows when negated.
                (-(number + 1) as $trgt) * 2 + 1
            }
        }

        #[cfg(test)]
        mod $test_name {
            use super::*;

            #[test]
            fn zero() {
                assert_eq!($name(0), 0);
            }

            #[test]
            fn positive() {
                assert_eq!($name(1), 2);
                assert_eq!($name(15), 30);
                assert_eq!($name(1234), 2468);
            }

            #[test]
            fn negative() {
                assert_eq!($name(-1), 1);
                assert_eq!($name(-15), 29);
                assert_eq!($name(-1234), 2467);
            }
        }
    }
}

// Not-string-based macros are great until this...
signed2unsigned_for_type!(signed2unsigned_16, i16, u16, type_16);
signed2unsigned_for_type!(signed2unsigned_32, i32, u32, type_32);
signed2unsigned_for_type!(signed2unsigned_64, i64, u64, type_64);
signed2unsigned_for_type!(signed2unsigned_128, i128, u128, type_128);

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
pub fn signed2unsigned(number: i64) -> u64 {
    signed2unsigned_64(number)
}

#[cfg(test)]
mod type_16_range {
    use super::*;

    #[test]
    fn maximum() {
        assert_eq!(signed2unsigned_16(::std::i16::MAX), ::std::u16::MAX - 1);
    }

    #[test]
    fn minimum() {
        assert_eq!(signed2unsigned_16(::std::i16::MIN), ::std::u16::MAX);
    }
}

#[cfg(test)]
mod type_32_range {
    use super::*;

    #[test]
    fn maximum() {
        assert_eq!(signed2unsigned_32(::std::i32::MAX), ::std::u32::MAX - 1);
    }

    #[test]
    fn minimum() {
        assert_eq!(signed2unsigned_32(::std::i32::MIN), ::std::u32::MAX);
    }
}

#[cfg(test)]
mod type_64_range {
    use super::*;

    #[test]
    fn maximum() {
        assert_eq!(signed2unsigned_64(::std::i64::MAX), ::std::u64::MAX - 1);
    }

    #[test]
    fn minimum() {
        assert_eq!(signed2unsigned_64(::std::i64::MIN), ::std::u64::MAX);
    }
}

#[cfg(test)]
mod type_128_range {
    use super::*;

    #[test]
    fn maximum() {
        assert_eq!(signed2unsigned_128(::std::i128::MAX), ::std::u128::MAX - 1);
    }

    #[test]
    fn minimum() {
        assert_eq!(signed2unsigned_128(::std::i128::MIN), ::std::u128::MAX);
    }
}