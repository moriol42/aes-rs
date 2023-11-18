use std::fs;

use clap::Parser;

use aes_rs::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = true)]
    encrypt: bool,

    #[arg(short, long)]
    decrypt: bool,

    /// Input file
    #[arg(short, long)]
    file: String,

    /// Output file
    #[arg(short, long)]
    out: Option<String>,

    /// Key file
    #[arg(short, long)]
    key: String,

    /// ECB mode
    #[arg(long, default_value_t = true)]
    ecb: bool,

    /// CBC mode
    #[arg(long, default_value_t = false)]
    cbc: bool,
}

/// Convert src to a vector of u8 of length 16
/// Example:
/// ```rust
/// let key = read_key("000102030405060708090A0B0C0D0E0F");
/// assert_eq!(Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), key);
/// ```
fn read_key(src: &str) -> Vec<u8> {
    // TODO: check key size
    let mut key = Vec::new();

    if src.len() < 32 {
        panic!("The key is to short");
    }

    for i in 0..16 {
        let x = match src.get(2 * i..2 * (i + 1)) {
            None => panic!("Bad key format"),
            Some(s) => u8::from_str_radix(s, 16).expect("Bad key format"),
        };
        key.push(x);
    }

    key
}

fn main() {
    let args = Args::parse();

    let file_path = args.file;
    let file = fs::read(&file_path).expect(&format!("Error cannot read file: {}", &file_path));

    let key_file =
        fs::read_to_string(&args.key).expect(&format!("Error cannot read file: {}", &args.key)); //str_to_state("MySuperSecretKey");
    let key = read_key(&key_file);

    let out_file_path = match args.out {
        None => {
            if args.decrypt {
                format!("{}.decr", &file_path)
            } else {
                format!("{}.enc", &file_path)
            }
        }
        Some(path) => path,
    };

    if args.decrypt {
        let decr_file = aes_decrypt_ecb(&file, &key);

        fs::write(&out_file_path, &decr_file)
            .expect(&format!("Error cannot write file: {}.decr", &file_path));
    } else {
        let enc_file = aes_encrypt_ecb(&file, &key);

        fs::write(&out_file_path, &enc_file)
            .expect(&format!("Error cannot write file: {}.enc", &file_path));
    }
}
