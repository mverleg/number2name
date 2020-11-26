use ::number2name::Charset;

pub fn charset_by_identifier(identifier: &str) -> Charset {
    assert!(identifier == "BASE64URL", "other charsets not yet implemented in CLI");

    unimplemented!();
}