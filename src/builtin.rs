use crate::charset::{Charset, Case};

const HEX: Charset = Charset::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567", Case::Sensitive);
const BASE32: Charset = Charset::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567", Case::Sensitive);
const BASE32HEX: Charset = Charset::new("0123456789ABCDEFGHIJKLMNOPQRSTUV", Case::Sensitive);
const BASE64: Charset = Charset::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/", Case::Sensitive);
const BASE64URL: Charset = Charset::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_", Case::Sensitive);
