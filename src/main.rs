fn str_to_matrix(text: &str) -> Vec<Vec<u8>> {
    /* Converts a 16-byte array into a 4x4 matrix. */
    let bytes = Vec::from_iter(text.bytes());
    let mut mat = Vec::new();

    for i in 0..4 {
        mat.push(Vec::from(&bytes[4 * i..4 * (i + 1)]));
    }

    mat
}

fn matrix_to_string(mat : Vec<Vec<u8>>) -> String {
    let mut s = String::new();
    for i in 0..4 {
        for j in 0..4 {
            s.push(char::from(mat[i][j]));
        }
    }
    s
}

fn main() {
    let text = "abcdefghijklmnop";
    let mat = str_to_matrix(text);
    println!("{:?}", mat);
    let s = matrix_to_string(mat);
    println!("{}", s);
}
