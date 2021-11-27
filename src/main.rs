use rand::prelude::*;
use std::fs;
use std::str;
use structopt::StructOpt;

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
}
fn is_ascii_printable(ch: u8) -> bool {
    if ch.is_ascii_punctuation() || ch.is_ascii_alphanumeric() {
        return true;
    }
    return false;
}

fn swap_case(c: u8, prob: f32) -> u8 {
    let out: u8;
    if c.is_ascii_lowercase() {
        out = c.to_ascii_uppercase();
    } else if c.is_ascii_uppercase() {
        out = c.to_ascii_lowercase();
    } else {
        out = c;
    }
    let t = random::<f32>();
    if t < prob {
        return out;
    }
    return c;
}

fn find_strings(f: Vec<u8>, len: usize, prob: f32) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    let mut string: Vec<u8> = Vec::new();

    for c in f {
        if is_ascii_printable(c) {
            let out = swap_case(c, prob);
            string.push(out);
            buf.push(out)
        } else {
            if string.len() > len {
                println!("{}", str::from_utf8(&string).unwrap());
            }
            string.clear();

            buf.push(c)
        }
    }
    return buf;
}

fn main() -> std::io::Result<()> {
    let opt = Cli::from_args();
    let f = fs::read(opt.in_path);

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    find_strings(f, opt.n, opt.p);

    return Ok(());
}
