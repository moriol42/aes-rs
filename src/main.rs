use std::env;
use std::fs;

use aes_rs::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let file_path = &args[1];
    let file = fs::read(file_path).expect(&format!("Error cannot read file: {}", file_path));

    let key = Vec::from([
        195, 44, 92, 166, 181, 128, 94, 12, 219, 141, 165, 122, 42, 182, 254, 92,
    ]);

    let enc_file = aes_encrypt_ecb(&file, &key);

    fs::write(&format!("{}.enc", file_path), &enc_file)
        .expect(&format!("Error cannot write file: {}.enc", file_path));
}
