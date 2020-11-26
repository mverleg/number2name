#[cfg(feature = "builtin-charsets")]
pub use crate::builtin::BASE32;
#[cfg(feature = "builtin-charsets")]
pub use crate::builtin::BASE32CROCKFORD;
#[cfg(feature = "builtin-charsets")]
pub use crate::builtin::BASE32HEX;
#[cfg(feature = "builtin-charsets")]
pub use crate::builtin::BASE32HUMAN;
#[cfg(feature = "builtin-charsets")]
pub use crate::builtin::BASE32LOWERCASE;
#[cfg(feature = "builtin-charsets")]
pub use crate::builtin::BASE32SCNY;
#[cfg(feature = "builtin-charsets")]
pub use crate::builtin::BASE64;
#[cfg(feature = "builtin-charsets")]
pub use crate::builtin::BASE64URL;
#[cfg(feature = "builtin-charsets")]
pub use crate::builtin::HEX;
#[cfg(feature = "builtin-charsets")]
pub use crate::builtin::HEXLOWERCASE;
pub use crate::charset::Charset;
pub use crate::decode::name2number;
pub use crate::decode::name2number_i128;
pub use crate::decode::name2number_i16;
pub use crate::decode::name2number_i32;
pub use crate::decode::name2number_i64;
pub use crate::decode::name2number_u128;
pub use crate::decode::name2number_u16;
pub use crate::decode::name2number_u32;
pub use crate::decode::name2number_u64;
pub use crate::encode::number2name;
pub use crate::encode::number2name_i128;
pub use crate::encode::number2name_i16;
pub use crate::encode::number2name_i32;
pub use crate::encode::number2name_i64;
pub use crate::encode::number2name_u128;
pub use crate::encode::number2name_u16;
pub use crate::encode::number2name_u32;
pub use crate::encode::number2name_u64;
pub use crate::signs::signed2unsigned;
pub use crate::signs::signed2unsigned_128;
pub use crate::signs::signed2unsigned_16;
pub use crate::signs::signed2unsigned_32;
pub use crate::signs::signed2unsigned_64;
pub use crate::signs::unsigned2signed;
pub use crate::signs::unsigned2signed_128;
pub use crate::signs::unsigned2signed_16;
pub use crate::signs::unsigned2signed_32;
pub use crate::signs::unsigned2signed_64;
pub use crate::typ::N2NErr;

#[cfg(feature = "builtin-charsets")]
mod builtin;
mod charset;
mod decode;
mod encode;
mod signs;
mod typ;
mod util;

#[cfg(test)]
mod tests {
    use crate::{Charset, N2NErr};

    #[test]
    fn demo() -> Result<(), N2NErr> {
        let charset = Charset::case_insensitive("abc");
        let text = charset.encode(13);
        assert_eq!(text, "aab");
        let nr = charset.decode(text)?;
        assert_eq!(nr, 13);
        Ok(())
    }
}
