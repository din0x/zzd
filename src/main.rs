use clap::Parser;
use std::{fs::File, io::Read, path::PathBuf};

#[derive(Parser)]
struct Args {
    file: PathBuf,
    /// Set the number of bytes per line in the output
    #[arg(short, long)]
    cols: Option<u8>,
    /// Output in plain hexdump style
    #[arg(short, long)]
    plain: bool,
    /// Switch to bits dump
    #[arg(short, long)]
    bits: bool,
    /// Start at <OFFSET> bytes
    #[arg(short = 's', long = "start")]
    offset: Option<usize>,
    /// Stop writing after <LEN> bytes
    #[arg(short = 'l', long = "len")]
    len: Option<usize>,
    /// Separate the output of every <BYTES> by whitespace
    #[arg(short = 'g', long = "groupsize")]
    bytes: Option<usize>,
}

fn main() {
    let args = Args::parse();

    let mut file = File::open(args.file).unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();

    let cols = args.cols.unwrap_or(16);

    let group = args.bytes.unwrap_or(if args.bits { 1 } else { 2 });

    let offset = args.offset.unwrap_or(0).min(bytes.len() - 1).min(bytes.len());
    let bytes = &bytes[offset..bytes.len()];
    let len = args.len.unwrap_or(bytes.len() - 1).clamp(0, bytes.len());
    let bytes = &bytes[0..len];

    hex_dump(
        &bytes,
        cols as usize,
        args.plain,
        group,
        args.bits,
        args.offset.unwrap_or(0),
    );
}

fn hex_dump(
    bytes: &[u8],
    cols: usize,
    plain: bool,
    group: usize,
    bits: bool,
    skip: usize,
) {
    let mut longest_chunk = 0;

    let mut line = skip;

    for chunk in bytes.chunks(cols) {
        if !plain {
            print!("{:08x}: ", line);
        }

        let mut chunk_size = 0;
        for group in chunk.chunks(group) {
            for byte in group {
                if bits {
                    print!("{:08b}", byte);
                    chunk_size += 8;
                } else {
                    print!("{:02x}", byte);
                    chunk_size += 2;
                }
            }

            if !plain {
                chunk_size += 1;
                print!(" ");
            }

            if chunk_size > longest_chunk {
                longest_chunk = chunk_size;
            }
        }

        print!(" ");

        if chunk.len() != cols {
            let spaces_to_print = longest_chunk - chunk_size;

            for _ in 0..spaces_to_print {
                print!(" ");
            }
        }

        if !plain {
            for ch in chunk.iter().copied() {
                match ch as char {
                    '\r' | '\n' | '\0' | '\t' => print!("."),
                    ch if ch.is_control() => print!("."),
                    _ => print!("{}", ch as char),
                }
            }
        }

        line += chunk.len();
        println!();
    }
}
