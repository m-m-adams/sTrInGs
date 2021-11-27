use super::Options;
use rand::prelude::*;
use std::error::Error;
use std::str;
use std::str::FromStr;
pub enum Encoding {
    UTF8,
    UTF16BE,
    UTF16LE,
}
// any error type implementing Display is acceptable.
type ParseError = &'static str;

impl FromStr for Encoding {
    type Err = ParseError;
    fn from_str(day: &str) -> Result<Self, Self::Err> {
        match day {
            "s" => Ok(Encoding::UTF8),
            "S" => Ok(Encoding::UTF8),
            "l" => Ok(Encoding::UTF16LE),
            "b" => Ok(Encoding::UTF16BE),
            _ => Err("Not an encoding option"),
        }
    }
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

pub fn find_strings_ascii(f: Vec<u8>, opt: &Options) {
    let mut string: Vec<u8> = Vec::new();
    let mut line = 0;
    for (count, c) in f.into_iter().enumerate() {
        if is_ascii_printable(c as char) {
            let out = swap_case(c as char, opt.prob);
            string.push(out);
        } else {
            if string.len() > opt.length {
                match opt.off {
                    true => println!("{:x}: {}", line, str::from_utf8(&string).unwrap()),
                    _ => println!("{}", str::from_utf8(&string).unwrap()),
                }
            }
            string.clear();
            line = count;
        }
    }
}

pub fn find_strings_utf16(f: Vec<u8>, options: &Options) -> Result<(), Box<dyn Error>> {
    let frombytes = match options.enc {
        Encoding::UTF16BE => u16::from_ne_bytes,
        Encoding::UTF16LE => u16::from_be_bytes,
        _ => return Err("not a valid encoding")?,
    };
    let fu16: Vec<u16> = f
        .chunks_exact(2)
        .into_iter()
        .map(|a| frombytes([a[0], a[1]]))
        .collect();

    let mut string: Vec<u8> = Vec::new();
    let mut line = 0;
    for (count, c) in char::decode_utf16(fu16).enumerate() {
        match c {
            Ok(c) => {
                if is_ascii_printable(c) {
                    let out = swap_case(c, options.prob);
                    string.push(out);
                } else {
                    if string.len() > options.length {
                        match options.off {
                            false => println!("{}", str::from_utf8(&string).unwrap()),
                            true => println!("{:x}: {}", line, str::from_utf8(&string).unwrap()),
                        }
                    }
                    string.clear();
                    line = count * 2;
                }
            }
            _ => {
                string.clear();
            }
        }
    }
    Ok(())
}
