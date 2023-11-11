use aes_rs::*;

fn main() {
    /*let state : Vec<Vec<u8>> = Vec::from([
        Vec::from([206, 243, 61, 34]),
        Vec::from([171, 11, 93, 31]),
        Vec::from([16, 200, 91, 108]),
        Vec::from([150, 3, 194, 51]),
    ]);

    let round_key : Vec<Vec<u8>> = Vec::from([
        Vec::from([173, 129, 68, 82]),
        Vec::from([223, 100, 38, 109]),
        Vec::from([32, 189, 53, 8]),
        Vec::from([253, 48, 187, 78]),
    ]);
    let res = add_round_key(state, round_key);
    println!("{:?}", res);
    println!("{}", matrix_to_string(res));*/

    /*let text = "abcdefghijklmnop";
    let mat = str_to_matrix(text);
    println!("{:?}", mat);
    let s = matrix_to_string(mat);

    println!("{}", s);*/

    let state = Vec::from([
        Vec::from([251, 64, 182, 81]),
        Vec::from([146, 168, 33, 80]),
        Vec::from([199, 159, 195, 24]),
        Vec::from([64, 80, 182, 255]),
    ]);

    println!("{}", matrix_to_string(sub_bytes(state, true)));
    
}
