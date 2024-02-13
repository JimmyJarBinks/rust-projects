use std::{
    fs::File,
    io::{BufReader, Read},
    ops::RangeInclusive,
};

use clap::{Parser, ValueEnum};
use regex::Regex;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct CliOptions {
    #[arg(help = "The path to the file")]
    path: std::path::PathBuf,

    #[arg(short, long, default_value="ASCII", value_parser=valid_encoding,
        help = "Parse the file with the given encoding [supports: ASCII, UTF-8]")]
    encoding: Encoding,

    #[arg(short, long,
        help = "Convert all strings to lowercase")]
    lowercase: bool,

    #[arg(short='m', long="match", default_value = "",
        help = "Define an exact match substring to filter strings")]
    substring: String,

    #[arg(short='n', long="bytes", default_value_t=4, value_parser=valid_word_length,
        help = "Minimum number of bytes considered a string")]
    min_len: u16,

    #[arg(short, long, default_value = "",
        help = "Define a regex pattern to filter strings")]
    regex: String,

    #[arg(short='t', long, default_value="n", value_parser=valid_radix,
        help = "Print the location of each string in (o)ctal, (d)ecimal, or he(x)")]
    radix: Radix,

    #[arg(short, long,
        help = "Only print unique strings (matching strings are merged to the first instance)")]
    uniq: bool,
}

#[derive(Debug)]
struct IndexedString {
    string: String,
    index: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Encoding {
    Ascii,
    Utf8,
}

fn valid_encoding(encoding: &str) -> Result<Encoding, String> {
    match encoding.to_lowercase().as_str() {
        "ascii" => Ok(Encoding::Ascii),
        "utf8" | "utf-8" => Ok(Encoding::Utf8),
        _ => Err(format!("acceptable encodings: ASCII, UTF-8")),   
    }
}

const PRINTABLE_ASCII_RANGE: RangeInclusive<u8> = 0x20..=0x7F;
const CONTROL_ASCII_RANGE: RangeInclusive<u8> = 0x00..=0x1F;

fn is_printable_ascii(byte: &u8) -> bool {
    PRINTABLE_ASCII_RANGE.contains(byte)
}

fn is_control_ascii(byte: &u8) -> bool {
    CONTROL_ASCII_RANGE.contains(byte)
}

fn is_valid_utf8(
    byte: &u8,
    reader: &mut BufReader<File>,
    read_buffer: &mut Vec<u8>,
    string_buffer: &mut Vec<u8>)
    -> bool {
    let mut bytes_left;
    if byte & 0b1111_1000 == 0b1111_0000 { bytes_left = 3; }
    else if byte & 0b1111_0000 == 0b1110_0000 { bytes_left = 2; }
    else if byte & 0b1110_0000 == 0b1100_0000 { bytes_left = 1; }
    else if byte & 0b1000_0000 == 0b0000_0000 { bytes_left = 0; }
    else { return false; }

    let mut bytes_read = 0;
    while bytes_left > 0 {
        if reader.read(read_buffer).is_ok() {
            if read_buffer[0] & 0b1100_0000 == 0b1000_0000 {
                string_buffer.push(read_buffer[0]);
                bytes_read += 1;
            }
            else {
                while bytes_read > 0 {
                    string_buffer.pop();
                    bytes_read -= 1;
                }
                return false;
            }
        }
        bytes_left -= 1;
    }
    true
}

fn valid_word_length(bytes: &str) -> Result<u16, String> {
    if let Ok(_) = bytes.parse::<f64>() {
        let parsed_number = bytes.parse::<u16>();
        if parsed_number.is_ok() && parsed_number.clone().unwrap() > 0 {
            return Ok(parsed_number.unwrap());
        }
        return Err(format!(
            "number of bytes must be an integer between 1 and 65535 (inclusive)"
        ));
    }
    Err(format!("{} is not numeric", bytes))
}

#[derive(Debug)]
struct RegexError(String);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Radix {
    Octal,
    Decimal,
    Hexadecimal,
    None,
}

fn valid_radix(radix: &str) -> Result<Radix, String> {
    match radix.to_lowercase().as_str() {
        "o" => Ok(Radix::Octal),
        "d" => Ok(Radix::Decimal),
        "x" => Ok(Radix::Hexadecimal),
        "n" => Ok(Radix::None),
        _ => Err(format!("acceptable values: o,d,x")),
    }
}

fn format_index(radix: Radix, index: usize) -> String {
    match radix {
        Radix::Octal => String::from(format!("{:o>7} ", index)),
        Radix::Decimal => String::from(format!("{:>7} ", index)),
        Radix::Hexadecimal => String::from(format!("{:x>7} ", index)),
        Radix::None => String::from(""),
    }
}

fn reset_str_buffer(buffer: &mut Vec<u8>, index: &mut usize) {
    *index += buffer.len() + 1;
    buffer.clear();
}

fn print_string(radix: &Radix, index: &usize, string: &String) {
    let displayed_index = format_index(*radix, *index);
    println!("{0}{1}", displayed_index, *string);
}

fn main() -> std::io::Result<()> {
    let args = CliOptions::parse();
    let file = File::open(args.path)?;
    let min_bytes: usize = args.min_len.into();
    let regex_obj = Regex::new(&args.regex);
    let regex_string = match regex_obj {
        Ok(regex_obj) => regex_obj,
        Err(_) =>  {
            println!("error: invalid regex expression \'{}\' for \'--regex <REGEX>\'", &args.regex);
            std::process::exit(1);
        },
    };
    let analysis_mode = !args.substring.is_empty() || !args.regex.is_empty() || args.uniq;

    let mut file_index: usize = 0;
    let mut reader = BufReader::new(file);
    let mut read_buffer: Vec<u8> = vec![0; 1];
    let mut string_buffer: Vec<u8> = Vec::new();
    let mut all_strings: Vec<IndexedString> = Vec::new();

    while let Ok(num_bytes) = reader.read(&mut read_buffer) {
        if num_bytes == 0 {
            break;
        }

        let byte: u8 = read_buffer[0];
        match args.encoding {
            Encoding::Ascii => {
                if is_printable_ascii(&byte) {
                    string_buffer.push(byte);
                    continue;
                }
            },
            Encoding::Utf8 => {
                if !is_control_ascii(&byte) {
                    string_buffer.push(byte);
                    if is_valid_utf8(&byte, &mut reader, &mut read_buffer, &mut string_buffer) {
                        continue;
                    }
                    string_buffer.pop();
                }
            },
        };

        let length_check = string_buffer.len() >= min_bytes;
        if !length_check {
            reset_str_buffer(&mut string_buffer, &mut file_index);
            continue;
        }

        if let Ok(found_string) = std::str::from_utf8(&string_buffer) {
            let mut output_string: String = found_string.to_owned();
            if args.lowercase {
                output_string = output_string.to_lowercase();
            }
            if analysis_mode {
                all_strings.push(IndexedString {
                    string: output_string,
                    index: file_index,
                });
            } else {
                print_string(&args.radix, &file_index, &output_string);
            }
        }
        reset_str_buffer(&mut string_buffer, &mut file_index);
    }

    if !all_strings.is_empty() {
        if args.uniq {
            all_strings.sort_by(|s1, s2| s1.string.cmp(&s2.string));
            all_strings.dedup_by(|s1, s2| s1.string.eq(&s2.string));
            all_strings.sort_by(|s1, s2| s1.index.cmp(&s2.index));
        }
        if !args.substring.is_empty() {
            all_strings.retain(|s| s.string.contains(&args.substring));
        }
        if !args.regex.is_empty() {
            
            all_strings.retain(|s| regex_string.is_match(&s.string));
        }
        for indexed_string in all_strings {
            print_string(&args.radix, &indexed_string.index, &indexed_string.string);
        }
    }
    Ok(())
}
