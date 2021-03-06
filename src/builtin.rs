use ::lazy_static::lazy_static;

use crate::charset::Charset;

lazy_static! {
    pub static ref HEX: Charset = Charset::case_insensitive("0123456789abcdef");
    pub static ref HEXLOWERCASE: Charset = Charset::case_sensitive("0123456789abcdef");
    pub static ref BASE32: Charset = Charset::case_insensitive("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567");
    pub static ref BASE32LOWERCASE: Charset = Charset::case_sensitive("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567");
    pub static ref BASE32HUMAN: Charset = Charset::case_insensitive("abcdefghjkmnpqrstuvwxyz23456789_");
    // Note: unlike 'real' Crockford, this does not accept e.g. L as 1 when decoding.
    pub static ref BASE32CROCKFORD: Charset = Charset::case_insensitive("0123456789ABCDEFGHJKMNPQRSTVWXYZ");
    pub static ref BASE32SCNY: Charset = Charset::case_sensitive("一二三四五六七八九十鼠牛虎兔龍蛇马羊猴鸡狗猪凤北东南西中左右上下");
    pub static ref BASE32HEX: Charset = Charset::case_insensitive("0123456789ABCDEFGHIJKLMNOPQRSTUV");
    pub static ref BASE64: Charset = Charset::case_sensitive("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
    pub static ref BASE64URL: Charset = Charset::case_sensitive("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex() {
        assert_eq!(HEX[0], '0');
        assert_eq!(HEX[15], 'f');
        assert_eq!(HEX.index_of('0').unwrap(), 0);
        assert_eq!(HEX.index_of('f').unwrap(), 15);
    }

    #[test]
    fn base32() {
        assert_eq!(BASE32[0], 'A');
        assert_eq!(BASE32[31], '7');
        assert_eq!(BASE32.index_of('A').unwrap(), 0);
        assert_eq!(BASE32.index_of('7').unwrap(), 31);
    }

    #[test]
    fn base32scny() {
        assert_eq!(BASE32SCNY[0], '一');
        assert_eq!(BASE32SCNY[31], '下');
        assert_eq!(BASE32SCNY.index_of('一').unwrap(), 0);
        assert_eq!(BASE32SCNY.index_of('下').unwrap(), 31);
    }

    #[test]
    fn base32human() {
        assert_eq!(BASE32HUMAN[0], 'a');
        assert_eq!(BASE32HUMAN[31], '_');
        assert_eq!(BASE32HUMAN.index_of('a').unwrap(), 0);
        assert_eq!(BASE32HUMAN.index_of('_').unwrap(), 31);
    }

    #[test]
    fn base32crockford() {
        assert_eq!(BASE32CROCKFORD[0], '0');
        assert_eq!(BASE32CROCKFORD[31], 'Z');
        assert_eq!(BASE32CROCKFORD.index_of('0').unwrap(), 0);
        assert_eq!(BASE32CROCKFORD.index_of('Z').unwrap(), 31);
    }

    #[test]
    fn base32hex() {
        assert_eq!(BASE32HEX[0], '0');
        assert_eq!(BASE32HEX[31], 'V');
        assert_eq!(BASE32HEX.index_of('0').unwrap(), 0);
        assert_eq!(BASE32HEX.index_of('V').unwrap(), 31);
    }

    #[test]
    fn base64() {
        assert_eq!(BASE64[0], 'A');
        assert_eq!(BASE64[63], '/');
        assert_eq!(BASE64.index_of('A').unwrap(), 0);
        assert_eq!(BASE64.index_of('/').unwrap(), 63);
    }

    #[test]
    fn base64url() {
        assert_eq!(BASE64URL[0], 'A');
        assert_eq!(BASE64URL[63], '_');
        assert_eq!(BASE64URL.index_of('A').unwrap(), 0);
        assert_eq!(BASE64URL.index_of('_').unwrap(), 63);
    }
}
