use ::std::io::stderr;
use ::std::io::Write;
use ::std::process::exit;

use ::number2name::name2number;
use ::structopt::StructOpt;

mod cli_util;

use crate::cli_util::charset_by_identifier;

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
        default_value = "BASE64URL",
        help = "Which character set to use, either name or quoted string"
    )]
    charset: String,

    #[structopt(
        short = "u",
        long,
        help = "Use unsigned encoding instead of signed"
    )]
    unsigned: bool,
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

    let charset = charset_by_identifier(&args.charset);

    for name in &args.names {
        if args.unsigned {
            //TODO @mark: u128
            let nr = name2number(name, &charset)
                .map_err(|err| err.to_string())?;
            println!("{}", nr);
        } else {
            //TODO @mark: i128
            let nr = name2number(name, &charset)
                .map_err(|err| err.to_string())?;
            println!("{}", nr);
        }
    }

    Ok(())
}
