mod builtin;
mod charset;
mod decode;
mod encode;
mod typ;
mod util;

pub use crate::builtin::BASE32;
pub use crate::builtin::BASE32CROCKFORD;
pub use crate::builtin::BASE32HEX;
pub use crate::builtin::BASE32HUMAN;
pub use crate::builtin::BASE32LOWERCASE;
pub use crate::builtin::BASE32SCNY;
pub use crate::builtin::BASE64;
pub use crate::builtin::BASE64URL;
pub use crate::builtin::HEX;
pub use crate::builtin::HEXLOWERCASE;
pub use crate::charset::Charset;
pub use crate::decode::name2number;
pub use crate::encode::number2name;
pub use crate::typ::N2NErr;

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
