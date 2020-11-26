use ::std::io::stderr;
use ::std::io::Write;
use ::std::process::exit;

use ::structopt::StructOpt;

use ::number2name::{number2name_i128, number2name_u128};

use crate::cli_util::charset_by_identifier;

mod cli_util;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "number2name",
    author = "github.com/mverleg/number2name",
    about = "Encode a string as a short number."
)]
pub struct Nr2NameArgs {
    #[structopt(
        name = "NUMBERS",
        required = true,
        min_values = 1,
        help = "One or more integer numbers (without thousand separators)"
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
        help = "Use signed encoding instead of unsigned (supporting negative numbers)"
    )]
    signed: bool,
}

pub fn main() {
    let args = Nr2NameArgs::from_args();
    if let Err(err) = go(&args) {
        stderr().write_all(err.as_bytes()).unwrap();
        stderr().write_all(b"\n").unwrap();
        exit(1);
    }
}

fn go(args: &Nr2NameArgs) -> Result<(), String> {

    let charset = charset_by_identifier(&args.charset)?;

    for nr_txt in &args.names {
        if args.signed {
            let nr: i128 = nr_txt.parse()
                .map_err(|_| format!("The input '{}' was not recognized as a valid number", &nr_txt))?;
            let txt = number2name_i128(nr, &charset);
            println!("{}", txt);
        } else {
            let nr: u128 = nr_txt.parse()
                .map_err(|_| format!("The input '{}' was not recognized as a valid unsigned number", &nr_txt))?;
            let txt = number2name_u128(nr, &charset);
            println!("{}", txt);
        }
    }

    Ok(())
}
