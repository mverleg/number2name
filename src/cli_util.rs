use ::number2name::BASE32;
use ::number2name::BASE32CROCKFORD;
use ::number2name::BASE32HEX;
use ::number2name::BASE32HUMAN;
use ::number2name::BASE32LOWERCASE;
use ::number2name::BASE32SCNY;
use ::number2name::BASE64;
use ::number2name::BASE64URL;
use ::number2name::Charset;
use ::number2name::HEX;
use ::number2name::HEXLOWERCASE;

pub fn charset_by_identifier(identifier: &str) -> Result<Charset, String> {
    Ok(match identifier.to_uppercase().as_str() {
        "HEX" => HEX.clone(),
        "HEXLOWERCASE" => HEXLOWERCASE.clone(),
        "BASE32" => BASE32.clone(),
        "BASE32LOWERCASE" => BASE32LOWERCASE.clone(),
        "BASE32HUMAN" => BASE32HUMAN.clone(),
        "BASE32CROCKFORD" => BASE32CROCKFORD.clone(),
        "BASE32SCNY" => BASE32SCNY.clone(),
        "BASE32HEX" => BASE32HEX.clone(),
        "BASE64" => BASE64.clone(),
        "BASE64URL" => BASE64URL.clone(),
        mut literal => {
            if literal.starts_with("'") || literal.starts_with("\"") {
                literal = literal[literal..literal.len()-1]
            } else {
                return Err(format!("Charset '{}' was not a built-in identifier and was not quoted; if you are providing a literal character set, enclose it in quotes (')", &literal))
            }
            unimplemented!()
        }
    })
}
