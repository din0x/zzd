use clap::Parser;
use std::{fs::File, io::Read, path::PathBuf};

#[derive(Parser)]
struct Args {
    file: PathBuf,
    #[arg(short, long)]
    cols: Option<u8>,
    #[arg(short, long)]
    plain: bool,
    #[arg(short, long)]
    bits: bool,
}

fn main() {
    let args = Args::parse();

    let mut file = File::open(args.file).unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();

    let cols = args.cols.unwrap_or(16);

    hex_dump(&bytes, cols as usize, args.plain, args.bits);
}

fn hex_dump(bytes: &[u8], cols: usize, plain: bool, bits: bool) {
    for chunk in bytes.chunks(cols) {
        for (i, byte) in chunk.iter().enumerate() {
            if bits {
                print!("{:08b}", byte)
            } else {
                print!("{:02x}", byte);
            }

            if i % 2 == 1 && !plain || bits {
                print!(" ");
            }
        }

        if plain {
            println!();
            continue;
        }

        print!("\x1b[{}G ", cols * 3 - cols / 2);

        for ch in chunk.iter().map(|el| *el) {
            match ch as char {
                '\r' | '\n' | '\0' | '\t' => print!("."),
                ch if ch.is_control() => print!("."),
                _ => print!("{}", ch as char),
            }
        }

        println!()
    }
}
