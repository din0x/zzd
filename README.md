# zzd
This project aims to replicate the functionality of the `xxd` command in Linux systems. `xxd` is a command-line utility that creates a hexadecimal dump of a given file.

## Install
```sh
git clone https://github.com/din0x/zzd.git;
cd zzd;
cargo build --release;
```

## Usage
`zzd [OPTIONS] <FILE>`
|      Option     |                 Description                  |
|-----------------|----------------------------------------------|
| `-c`, `--cols`  |Set the number of bytes per line in the output|
| `-p`, `--plain` |Output in plain hexdump style                 |
| `-b`, `--bits`  |Switch to bits dump                           |
| `-h`, `--help`  |Print help                                    |
