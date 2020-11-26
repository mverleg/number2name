use ::std::io::stderr;
use ::std::io::Write;
use ::std::process::exit;

use ::structopt::StructOpt;

use ::number2name::name2number;

use crate::cli_util::charset_by_identifier;

mod cli_util;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "name2number",
    author = "github.com/mverleg/number2name",
    about = "Decode a string encoded by number2name back to a number."
)]
pub struct Name2NrArgs {
    #[structopt(
        name = "NAMES",
        required = true,
        min_values = 1,
        help = "One or more encoded names"
    )]
    names: Vec<String>,

    #[structopt(
        short = "c",
        long,
        default_value = "BASE32HUMAN",
        help = "Which (case-sensitive) character set to use, either name or quoted string"
    )]
    charset: String,

    #[structopt(
        short = "s",
        long,
        help = "Use signed decoding instead of unsigned (has to match the option used when encoding)"
    )]
    signed: bool,
}

pub fn main() {
    let args = Name2NrArgs::from_args();
    if let Err(err) = go(&args) {
        stderr().write_all(err.as_bytes()).unwrap();
        stderr().write_all(b"\n").unwrap();
        exit(1);
    }
}

fn go(args: &Name2NrArgs) -> Result<(), String> {

    let charset = charset_by_identifier(&args.charset)?;

    for name in &args.names {
        if args.signed {
            let nr = name2number_i128(name, &charset)
                .map_err(|err| err.to_string())?;
            println!("{}", nr);
        } else {
            //TODO @mark: u128
            let nr = name2number_u128(name, &charset)
                .map_err(|err| err.to_string())?;
            println!("{}", nr);
        }
    }

    Ok(())
}
