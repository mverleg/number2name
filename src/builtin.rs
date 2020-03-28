use ::lazy_static::lazy_static;
use crate::charset::{Charset, Case};

lazy_static! {
    pub static ref HEX: Charset = Charset::new("0123456789abcdef", Case::Insensitive);
    pub static ref BASE32: Charset = Charset::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567", Case::Insensitive);
    pub static ref BASE32HUMAN: Charset = Charset::new("abcdefghjkmnpqrstuvwxyz23456789_", Case::Insensitive);
    pub static ref BASE32SCNY: Charset = Charset::new("一二三四五六七八九十鼠牛虎兔龍蛇马羊猴鸡狗猪凤北东南西中左右上下", Case::Insensitive);
    pub static ref BASE32HEX: Charset = Charset::new("0123456789ABCDEFGHIJKLMNOPQRSTUV", Case::Insensitive);
    pub static ref BASE64: Charset = Charset::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/", Case::Sensitive);
    pub static ref BASE64URL: Charset = Charset::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_", Case::Sensitive);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex() {
        assert_eq!(HEX[0], '0');
        assert_eq!(HEX[15], 'f');
    }

    #[test]
    fn base32() {
        assert_eq!(BASE32[0], 'A');
        assert_eq!(BASE32[31], '7');
    }

    #[test]
    fn base32scny() {
        assert_eq!(BASE32SCNY[0], '一');
        assert_eq!(BASE32SCNY[31], '下');
    }

    #[test]
    fn base32human() {
        assert_eq!(BASE32HUMAN[0], 'a');
        assert_eq!(BASE32HUMAN[31], '_');
    }

    #[test]
    fn base32hex() {
        assert_eq!(BASE32HEX[0], '0');
        assert_eq!(BASE32HEX[31], 'V');
    }

    #[test]
    fn base64() {
        assert_eq!(BASE64[0], 'A');
        assert_eq!(BASE64[63], '/');
    }

    #[test]
    fn base64url() {
        assert_eq!(BASE64URL[0], 'A');
        assert_eq!(BASE64URL[63], '_');
    }
}
