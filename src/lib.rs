
mod util;
mod typ;
mod charset;
mod builtin;
mod encode;
mod decode;

pub use crate::charset::Charset;
pub use crate::typ::N2NErr;
pub use crate::encode::number2name;
pub use crate::decode::name2number;
pub use crate::builtin::HEX;
pub use crate::builtin::BASE32;
pub use crate::builtin::BASE32HUMAN;
pub use crate::builtin::BASE32SCNY;
pub use crate::builtin::BASE32HEX;
pub use crate::builtin::BASE64;
pub use crate::builtin::BASE64URL;

#[cfg(test)]
mod tests {
    use crate::Charset;

    #[test]
    fn demo() {
        let charset = Charset::case_insensitive("abc");
        let text = charset.encode(14);
        assert_eq!(text, "aab");
        //TODO @mark: remove unwraps from tests
        let nr = charset.decode(text).unwrap();
        assert_eq!(nr, 14);
    }
}
