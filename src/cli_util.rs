use ::number2name::BASE64URL;
use ::number2name::Charset;

pub fn charset_by_identifier(identifier: &str) -> Charset {
    if identifier == "BASE64URL" {
        return BASE64URL.clone()
    }
    unimplemented!("other charsets not yet implemented in CLI");
}
