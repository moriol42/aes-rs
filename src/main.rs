//use std::env;
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

    #[arg(short, long)]
    file: String,
}

fn main() {
    //let args: Vec<String> = env::args().collect();

    let args = Args::parse();

    let file_path = args.file;
    let file = fs::read(&file_path).expect(&format!("Error cannot read file: {}", &file_path));

    //let key = Vec::from([
    //    195, 44, 92, 166, 181, 128, 94, 12, 219, 141, 165, 122, 42, 182, 254, 92,
    //]);
    let key = str_to_state("MySuperSecretKey");

    if args.decrypt {
        let decr_file = aes_decrypt_ecb(&file, &key);

        fs::write(&format!("{}.decr", &file_path), &decr_file)
            .expect(&format!("Error cannot write file: {}.decr", &file_path));
    } else {
        let enc_file = aes_encrypt_ecb(&file, &key);

        fs::write(&format!("{}.enc", &file_path), &enc_file)
            .expect(&format!("Error cannot write file: {}.enc", &file_path));
    }
}
