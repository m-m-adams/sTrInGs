mod strings;
use std::error::Error;
use std::fmt;
use std::fs;
use strings::Encoding;
pub use structopt::StructOpt;

/// Search for strings in a file and print them sarcastically
#[derive(StructOpt)]
pub struct Cli {
    /// The path to read from
    #[structopt(parse(from_os_str))]
    pub in_path: std::path::PathBuf,

    /// minimum length to search for
    #[structopt(short = "n", long = "--bytes", default_value = "4")]
    pub length: usize,

    /// Print the hex offset within the file before each string
    #[structopt(short = "o", long = "--offset")]
    pub offset: bool,

    ///probability of swapping case
    #[structopt(short = "p", long = "--swap-prob", alias = "-", default_value = "0.5")]
    pub prob: f32,
    ///Select character size and endianness:
    ///s = 7-bit, S = 8-bit, {b,l} = 16-bit, {B,L} = 32-bit
    #[structopt(short = "e", long = "--encoding", possible_values=&["s", "S", "b", "l"], default_value="S")]
    pub enc: Encoding,
}

pub struct Options {
    pub length: usize,
    pub prob: f32,
    pub enc: Encoding,
    pub off: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::from_args();
    let f = fs::read(cli.in_path);

    let opt: Options = Options {
        length: cli.length,
        prob: cli.prob,
        enc: cli.enc,
        off: cli.offset,
    };

    let f = match f {
        Ok(file) => file,
        Err(error) => return Err(format!("Problem opening the file: {:?}", error.kind()))?,
    };
    match opt.enc {
        strings::Encoding::UTF8 => strings::find_strings_ascii(f, &opt),
        _ => strings::find_strings_utf16(f, &opt)?,
    }

    return Ok(());
}
