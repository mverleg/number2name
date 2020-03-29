use ::lazy_static::lazy_static;
use crate::charset::{Charset, Case};

lazy_static! {
    pub static ref HEX: Charset = Charset::new("0123456789abcdef", Case::Insensitive);
    pub static ref BASE32: Charset = Charset::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567", Case::Insensitive);
    pub static ref BASE32HUMAN: Charset = Charset::new("abcdefghjkmnpqrstuvwxyz23456789_", Case::Insensitive);
    // Note: unlike 'real' Crockford, this does not accept e.g. L as 1 when decoding.
    pub static ref BASE32CROCKFORD: Charset = Charset::new("0123456789ABCDEFGHJKMNPQRSTVWXYZ", Case::Insensitive);
    pub static ref BASE32SCNY: Charset = Charset::new("一二三四五六七八九十鼠牛虎兔龍蛇马羊猴鸡狗猪凤北东南西中左右上下", Case::Insensitive);
    pub static ref BASE32HEX: Charset = Charset::new("0123456789ABCDEFGHIJKLMNOPQRSTUV", Case::Insensitive);
    pub static ref BASE64: Charset = Charset::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/", Case::Sensitive);
    pub static ref BASE64URL: Charset = Charset::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_", Case::Sensitive);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex() -> Result<(), ()> {
        assert_eq!(HEX[0], '0');
        assert_eq!(HEX[15], 'f');
        assert_eq!(HEX.index_of('0')?, 0);
        assert_eq!(HEX.index_of('f')?, 15);
        Ok(())
    }

    #[test]
    fn base32() -> Result<(), ()> {
        assert_eq!(BASE32[0], 'A');
        assert_eq!(BASE32[31], '7');
        assert_eq!(BASE32.index_of('A')?, 0);
        assert_eq!(BASE32.index_of('7')?, 31);
        Ok(())
    }

    #[test]
    fn base32scny() -> Result<(), ()> {
        assert_eq!(BASE32SCNY[0], '一');
        assert_eq!(BASE32SCNY[31], '下');
        assert_eq!(BASE32SCNY.index_of('一')?, 0);
        assert_eq!(BASE32SCNY.index_of('下')?, 31);
        Ok(())
    }

    #[test]
    fn base32human() -> Result<(), ()> {
        assert_eq!(BASE32HUMAN[0], 'a');
        assert_eq!(BASE32HUMAN[31], '_');
        assert_eq!(BASE32HUMAN.index_of('a')?, 0);
        assert_eq!(BASE32HUMAN.index_of('_')?, 31);
        Ok(())
    }

    #[test]
    fn base32crockford() -> Result<(), ()> {
        assert_eq!(BASE32CROCKFORD[0], '0');
        assert_eq!(BASE32CROCKFORD[31], 'Z');
        assert_eq!(BASE32CROCKFORD.index_of('0')?, 0);
        assert_eq!(BASE32CROCKFORD.index_of('Z')?, 31);
        Ok(())
    }

    #[test]
    fn base32hex() -> Result<(), ()> {
        assert_eq!(BASE32HEX[0], '0');
        assert_eq!(BASE32HEX[31], 'V');
        assert_eq!(BASE32HEX.index_of('0')?, 0);
        assert_eq!(BASE32HEX.index_of('V')?, 31);
        Ok(())
    }

    #[test]
    fn base64() -> Result<(), ()> {
        assert_eq!(BASE64[0], 'A');
        assert_eq!(BASE64[63], '/');
        assert_eq!(BASE64.index_of('A')?, 0);
        assert_eq!(BASE64.index_of('/')?, 63);
        Ok(())
    }

    #[test]
    fn base64url() -> Result<(), ()> {
        assert_eq!(BASE64URL[0], 'A');
        assert_eq!(BASE64URL[63], '_');
        assert_eq!(BASE64URL.index_of('A')?, 0);
        assert_eq!(BASE64URL.index_of('_')?, 63);
        Ok(())
    }
}
