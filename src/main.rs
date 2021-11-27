use rand::prelude::*;
use std::error::Error;
use std::fs;
use std::str;
use std::str::FromStr;
use structopt::StructOpt;
enum Encodings {
    UTF8,
    UTF16BE,
    UTF16LE,
}
// any error type implementing Display is acceptable.
type ParseError = &'static str;

impl FromStr for Encodings {
    type Err = ParseError;
    fn from_str(day: &str) -> Result<Self, Self::Err> {
        match day {
            "s" => Ok(Encodings::UTF8),
            "S" => Ok(Encodings::UTF8),
            "l" => Ok(Encodings::UTF16LE),
            "b" => Ok(Encodings::UTF16BE),
            _ => Err("Not an encoding option"),
        }
    }
}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to read from
    #[structopt(parse(from_os_str))]
    in_path: std::path::PathBuf,

    /// length to search for
    #[structopt(short = "n", long = "--bytes", default_value = "4")]
    n: usize,

    ///prob of swapping case
    #[structopt(short = "p", long = "--swap-prob", default_value = "0.5")]
    p: f32,

    ///encoding
    #[structopt(short = "e", long = "--encoding", possible_values=&["s", "S", "b", "l"], default_value="S")]
    e: Encodings,
}
fn is_ascii_printable(ch: char) -> bool {
    if ch.is_ascii_punctuation() || ch.is_ascii_alphanumeric() {
        return true;
    }
    return false;
}

fn swap_case(c: char, prob: f32) -> u8 {
    let out: char;
    if c.is_ascii_lowercase() {
        out = c.to_ascii_uppercase();
    } else if c.is_ascii_uppercase() {
        out = c.to_ascii_lowercase();
    } else {
        out = c;
    }
    let t = random::<f32>();
    if t < prob {
        return out as u8;
    }
    return c as u8;
}

fn find_strings_ascii(f: Vec<u8>, len: usize, prob: f32) {
    let mut string: Vec<u8> = Vec::new();
    for c in f {
        if is_ascii_printable(c as char) {
            let out = swap_case(c as char, prob);
            string.push(out);
        } else {
            if string.len() > len {
                println!("{}", str::from_utf8(&string).unwrap());
            }
            string.clear();
        }
    }
}

fn find_strings_utf16(f: Vec<u8>, le: Encodings, len: usize, prob: f32) -> Result<(), ParseError> {
    let frombytes = match le {
        Encodings::UTF16BE => u16::from_ne_bytes,
        Encodings::UTF16LE => u16::from_be_bytes,
        _ => return Err("not a valid encoding"),
    };
    let fu16: Vec<u16> = f
        .chunks_exact(2)
        .into_iter()
        .map(|a| frombytes([a[0], a[1]]))
        .collect();

    let mut string: Vec<u8> = Vec::new();
    for c in char::decode_utf16(fu16) {
        match c {
            Ok(c) => {
                if is_ascii_printable(c) {
                    let out = swap_case(c, prob);
                    string.push(out);
                } else {
                    if string.len() > len {
                        println!("{}", str::from_utf8(&string).unwrap());
                    }
                    string.clear();
                }
            }
            _ => {
                string.clear();
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Cli::from_args();
    let f = fs::read(opt.in_path);

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    match opt.e {
        Encodings::UTF8 => find_strings_ascii(f, opt.n, opt.p),
        _ => find_strings_utf16(f, opt.e, opt.n, opt.p)?,
    }

    return Ok(());
}
