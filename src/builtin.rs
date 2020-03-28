use ::lazy_static::lazy_static;
use crate::charset::{Charset, Case};

lazy_static! {
    pub static ref HUMANHEX: Charset = Charset::new("abcdefghjkmnpqrstuvwxyz23456789_", Case::Insensitive);
    pub static ref HEX: Charset = Charset::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567", Case::Insensitive);
    pub static ref BASE32: Charset = Charset::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567", Case::Insensitive);
    pub static ref BASE32HEX: Charset = Charset::new("0123456789ABCDEFGHIJKLMNOPQRSTUV", Case::Insensitive);
    pub static ref BASE64: Charset = Charset::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/", Case::Sensitive);
    pub static ref BASE64URL: Charset = Charset::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_", Case::Sensitive);
}
