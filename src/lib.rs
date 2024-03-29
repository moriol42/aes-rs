const S_BOX: [u8; 256] = [
    0x63, 0x7C, 0x77, 0x7B, 0xF2, 0x6B, 0x6F, 0xC5, 0x30, 0x01, 0x67, 0x2B, 0xFE, 0xD7, 0xAB, 0x76,
    0xCA, 0x82, 0xC9, 0x7D, 0xFA, 0x59, 0x47, 0xF0, 0xAD, 0xD4, 0xA2, 0xAF, 0x9C, 0xA4, 0x72, 0xC0,
    0xB7, 0xFD, 0x93, 0x26, 0x36, 0x3F, 0xF7, 0xCC, 0x34, 0xA5, 0xE5, 0xF1, 0x71, 0xD8, 0x31, 0x15,
    0x04, 0xC7, 0x23, 0xC3, 0x18, 0x96, 0x05, 0x9A, 0x07, 0x12, 0x80, 0xE2, 0xEB, 0x27, 0xB2, 0x75,
    0x09, 0x83, 0x2C, 0x1A, 0x1B, 0x6E, 0x5A, 0xA0, 0x52, 0x3B, 0xD6, 0xB3, 0x29, 0xE3, 0x2F, 0x84,
    0x53, 0xD1, 0x00, 0xED, 0x20, 0xFC, 0xB1, 0x5B, 0x6A, 0xCB, 0xBE, 0x39, 0x4A, 0x4C, 0x58, 0xCF,
    0xD0, 0xEF, 0xAA, 0xFB, 0x43, 0x4D, 0x33, 0x85, 0x45, 0xF9, 0x02, 0x7F, 0x50, 0x3C, 0x9F, 0xA8,
    0x51, 0xA3, 0x40, 0x8F, 0x92, 0x9D, 0x38, 0xF5, 0xBC, 0xB6, 0xDA, 0x21, 0x10, 0xFF, 0xF3, 0xD2,
    0xCD, 0x0C, 0x13, 0xEC, 0x5F, 0x97, 0x44, 0x17, 0xC4, 0xA7, 0x7E, 0x3D, 0x64, 0x5D, 0x19, 0x73,
    0x60, 0x81, 0x4F, 0xDC, 0x22, 0x2A, 0x90, 0x88, 0x46, 0xEE, 0xB8, 0x14, 0xDE, 0x5E, 0x0B, 0xDB,
    0xE0, 0x32, 0x3A, 0x0A, 0x49, 0x06, 0x24, 0x5C, 0xC2, 0xD3, 0xAC, 0x62, 0x91, 0x95, 0xE4, 0x79,
    0xE7, 0xC8, 0x37, 0x6D, 0x8D, 0xD5, 0x4E, 0xA9, 0x6C, 0x56, 0xF4, 0xEA, 0x65, 0x7A, 0xAE, 0x08,
    0xBA, 0x78, 0x25, 0x2E, 0x1C, 0xA6, 0xB4, 0xC6, 0xE8, 0xDD, 0x74, 0x1F, 0x4B, 0xBD, 0x8B, 0x8A,
    0x70, 0x3E, 0xB5, 0x66, 0x48, 0x03, 0xF6, 0x0E, 0x61, 0x35, 0x57, 0xB9, 0x86, 0xC1, 0x1D, 0x9E,
    0xE1, 0xF8, 0x98, 0x11, 0x69, 0xD9, 0x8E, 0x94, 0x9B, 0x1E, 0x87, 0xE9, 0xCE, 0x55, 0x28, 0xDF,
    0x8C, 0xA1, 0x89, 0x0D, 0xBF, 0xE6, 0x42, 0x68, 0x41, 0x99, 0x2D, 0x0F, 0xB0, 0x54, 0xBB, 0x16,
];

const INV_S_BOX: [u8; 256] = [
    0x52, 0x09, 0x6A, 0xD5, 0x30, 0x36, 0xA5, 0x38, 0xBF, 0x40, 0xA3, 0x9E, 0x81, 0xF3, 0xD7, 0xFB,
    0x7C, 0xE3, 0x39, 0x82, 0x9B, 0x2F, 0xFF, 0x87, 0x34, 0x8E, 0x43, 0x44, 0xC4, 0xDE, 0xE9, 0xCB,
    0x54, 0x7B, 0x94, 0x32, 0xA6, 0xC2, 0x23, 0x3D, 0xEE, 0x4C, 0x95, 0x0B, 0x42, 0xFA, 0xC3, 0x4E,
    0x08, 0x2E, 0xA1, 0x66, 0x28, 0xD9, 0x24, 0xB2, 0x76, 0x5B, 0xA2, 0x49, 0x6D, 0x8B, 0xD1, 0x25,
    0x72, 0xF8, 0xF6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xD4, 0xA4, 0x5C, 0xCC, 0x5D, 0x65, 0xB6, 0x92,
    0x6C, 0x70, 0x48, 0x50, 0xFD, 0xED, 0xB9, 0xDA, 0x5E, 0x15, 0x46, 0x57, 0xA7, 0x8D, 0x9D, 0x84,
    0x90, 0xD8, 0xAB, 0x00, 0x8C, 0xBC, 0xD3, 0x0A, 0xF7, 0xE4, 0x58, 0x05, 0xB8, 0xB3, 0x45, 0x06,
    0xD0, 0x2C, 0x1E, 0x8F, 0xCA, 0x3F, 0x0F, 0x02, 0xC1, 0xAF, 0xBD, 0x03, 0x01, 0x13, 0x8A, 0x6B,
    0x3A, 0x91, 0x11, 0x41, 0x4F, 0x67, 0xDC, 0xEA, 0x97, 0xF2, 0xCF, 0xCE, 0xF0, 0xB4, 0xE6, 0x73,
    0x96, 0xAC, 0x74, 0x22, 0xE7, 0xAD, 0x35, 0x85, 0xE2, 0xF9, 0x37, 0xE8, 0x1C, 0x75, 0xDF, 0x6E,
    0x47, 0xF1, 0x1A, 0x71, 0x1D, 0x29, 0xC5, 0x89, 0x6F, 0xB7, 0x62, 0x0E, 0xAA, 0x18, 0xBE, 0x1B,
    0xFC, 0x56, 0x3E, 0x4B, 0xC6, 0xD2, 0x79, 0x20, 0x9A, 0xDB, 0xC0, 0xFE, 0x78, 0xCD, 0x5A, 0xF4,
    0x1F, 0xDD, 0xA8, 0x33, 0x88, 0x07, 0xC7, 0x31, 0xB1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xEC, 0x5F,
    0x60, 0x51, 0x7F, 0xA9, 0x19, 0xB5, 0x4A, 0x0D, 0x2D, 0xE5, 0x7A, 0x9F, 0x93, 0xC9, 0x9C, 0xEF,
    0xA0, 0xE0, 0x3B, 0x4D, 0xAE, 0x2A, 0xF5, 0xB0, 0xC8, 0xEB, 0xBB, 0x3C, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2B, 0x04, 0x7E, 0xBA, 0x77, 0xD6, 0x26, 0xE1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0C, 0x7D,
];

pub fn str_to_state(text: &str) -> Vec<u8> {
    /* Converts text into a vector of u8 of length 16. */
    Vec::from_iter(text.bytes())
}

pub fn state_to_string(state: &Vec<u8>) -> String {
    /* Convert a vector of length 16 of u8 into a string */
    state.iter().map(|c| *c as char).collect::<String>()
}

pub fn add_round_key(state: &mut Vec<u8>, round_key: &Vec<u8>) -> () {
    /* XOR the vector of u8 of length 16 state with the round key (vector of u8 of length 16) */
    for i in 0..16 {
        state[i] ^= round_key[i];
    }
}

pub fn sub_bytes(state: &mut Vec<u8>, inv: bool) -> () {
    /* Bytes substitution */
    let s = if inv { INV_S_BOX } else { S_BOX };
    let n = state.len();
    for i in 0..n {
        state[i] = s[state[i] as usize];
    }
}

pub fn shift_rows(state: &mut Vec<u8>) -> () {
    /* Shift rows of state. Warning: rows and columns are inverted */
    for i in 1..4 {
        let bak = [state[i], state[i + 4], state[i + 8], state[i + 12]];
        for j in 0..4 {
            state[i + 4 * j] = bak[(j + i) % 4];
        }
    }
}

pub fn inv_shift_rows(state: &mut Vec<u8>) -> () {
    /* Reverse the shift of the rows of state. Warning: rows and columns are inverted */
    for i in 1..4 {
        let bak = [state[i], state[i + 4], state[i + 8], state[i + 12]];
        for j in 0..4 {
            state[i + 4 * j] = bak[(j + 4 - i) % 4];
        }
    }
}

fn gal_mul(mut x: u8, mut y: u8) -> u8 {
    /* Multiplication in galois field GF(2^8) */
    let mut p = 0;
    while x != 0 && y != 0 {
        if y & 1 != 0 {
            p ^= x;
        }
        if x & 0x80 != 0 {
            x = (x << 1) ^ 0x1b;
        } else {
            x <<= 1;
        }
        y >>= 1;
    }
    p
}

const MIX: [[u8; 4]; 4] = [[2, 1, 1, 3], [3, 2, 1, 1], [1, 3, 2, 1], [1, 1, 3, 2]];

const INV_MIX: [[u8; 4]; 4] = [
    [14, 9, 13, 11],
    [11, 14, 9, 13],
    [13, 11, 14, 9],
    [9, 13, 11, 14],
];

pub fn mix_columns(state: &mut Vec<u8>, inv: bool) -> () {
    /* Mix the columns by doing state^T x mix_state (since rows
    and columns are inverted) */
    let mix_state = if inv { INV_MIX } else { MIX };
    let mut state2 = Vec::new();
    for i in 0..4 {
        for j in 0..4 {
            let mut x = 0;
            for k in 0..4 {
                x ^= gal_mul(state[4 * i + k], mix_state[k][j]);
            }
            state2.push(x);
        }
    }
    *state = state2;
}

pub fn aes_encrypt_block(block: &Vec<u8>, expanded_key: &Vec<Vec<u8>>) -> Vec<u8> {
    /* Encrypt a 16 bytes block with the expanded key expanded_key using aes */
    let mut state = block.clone();
    add_round_key(&mut state, &expanded_key[0]);
    for i in 1..10 {
        sub_bytes(&mut state, false);
        shift_rows(&mut state);
        mix_columns(&mut state, false);
        add_round_key(&mut state, &expanded_key[i]);
    }
    sub_bytes(&mut state, false);
    shift_rows(&mut state);
    add_round_key(&mut state, &expanded_key[10]);

    state
}

pub fn aes_decrypt_block(block: &Vec<u8>, expanded_key: &Vec<Vec<u8>>) -> Vec<u8> {
    /* Decrypt a 16 bytes block with the expanded key expanded_key using aes */
    let mut state = block.clone();
    add_round_key(&mut state, &expanded_key[10]);
    inv_shift_rows(&mut state);
    sub_bytes(&mut state, true);
    for i in (1..10).rev() {
        add_round_key(&mut state, &expanded_key[i]);
        mix_columns(&mut state, true);
        inv_shift_rows(&mut state);
        sub_bytes(&mut state, true);
    }
    add_round_key(&mut state, &expanded_key[0]);

    state
}

pub fn rot_word(word: &mut Vec<u8>) -> () {
    let len = 4;
    let bak = word[0];
    for i in 0..(len - 1) {
        word[i] = word[(i + 1) % len];
    }
    word[len - 1] = bak;
}

pub fn key_expansion(key: &Vec<u8>) -> Vec<Vec<u8>> {
    let r_con = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36];
    let mut exp_key = Vec::new();
    exp_key.push(key.clone());

    for i in 0..10 {
        let mut w = Vec::from(&exp_key[i][12..16]);
        rot_word(&mut w);
        sub_bytes(&mut w, false);
        w[0] ^= r_con[i];
        let mut round_key: Vec<u8> = Vec::new();
        for k in 0..4 {
            for j in 0..4 {
                w[j] ^= exp_key[i][j + 4 * k];
            }
            round_key.extend(&w);
        }
        exp_key.push(round_key);
    }

    exp_key
}

pub fn aes_encrypt_ecb(msg: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    let n = msg.len();
    let exp_key = key_expansion(&key);
    let mut enc_msg = Vec::new();
    let n_blocks = n / 16;

    for i in 0..n_blocks {
        let mut enc_block = aes_encrypt_block(&Vec::from(&msg[16 * i..16 * (i + 1)]), &exp_key);
        enc_msg.append(&mut enc_block);
    }

    let r = n % 16;
    let mut end_msg = Vec::from(&msg[16 * n_blocks..16 * n_blocks + r]);
    end_msg.extend(&vec![(16 - r) as u8; 16 - r]);
    let mut enc_block = aes_encrypt_block(&end_msg, &exp_key);
    enc_msg.append(&mut enc_block);

    enc_msg
}

pub fn aes_decrypt_ecb(msg: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    let n = msg.len();
    let exp_key = key_expansion(&key);
    let mut decr_msg = Vec::new();
    let n_blocks = n / 16;

    if n % 16 != 0 {
        panic!("");
    }

    for i in 0..n_blocks {
        let mut decr_block = aes_decrypt_block(&Vec::from(&msg[16 * i..16 * (i + 1)]), &exp_key);

        if i == n_blocks - 1 {
            let r = decr_block.pop().unwrap();
            for _ in 0..(r - 1) {
                let x = decr_block.pop().unwrap();
                assert_eq!(x, r);
            }
        }
        decr_msg.append(&mut decr_block);
    }

    decr_msg
}

pub fn aes_encrypt_cbc(msg: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    let n = msg.len();
    let exp_key = key_expansion(&key);
    let mut enc_msg = Vec::new();

    // Add the IV (initilization vector) to enc_msg
    // TODO: choose a random one
    for i in 0..16 {
        enc_msg.push(i);
    }
    let n_blocks = n / 16;

    for i in 0..n_blocks {
        let mut block = Vec::from(&msg[16 * i..16 * (i + 1)]);
        let rk = Vec::from(&enc_msg[16 * i..16 * (i + 1)]);
        add_round_key(&mut block, &rk);
        let mut enc_block = aes_encrypt_block(&block, &exp_key);
        enc_msg.append(&mut enc_block);
    }

    let r = n % 16;
    let mut end_msg = Vec::from(&msg[16 * n_blocks..16 * n_blocks + r]);
    end_msg.extend(&vec![(16 - r) as u8; 16 - r]);
    add_round_key(&mut end_msg, &Vec::from(&enc_msg[16 * n_blocks..16 * (n_blocks + 1)]));
    let mut enc_block = aes_encrypt_block(&end_msg, &exp_key);
    enc_msg.append(&mut enc_block);

    enc_msg
}

pub fn aes_decrypt_cbc(msg: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    let n = msg.len();
    let exp_key = key_expansion(&key);
    let mut dec_msg = Vec::new();

    let n_blocks = n / 16;
    // The first block is the IV, thus the for loop ends at 1
    for i in (1..n_blocks).rev() {
        let block = Vec::from(&msg[16 * i..16 * (i + 1)]);
        let mut decr_block = aes_decrypt_block(&block, &exp_key);
        let rk = Vec::from(&msg[16 * (i - 1)..16 * i]);
        add_round_key(&mut decr_block, &rk);

        if i == n_blocks - 1 {
            let r = decr_block.pop().unwrap();
            for _ in 0..(r - 1) {
                let x = decr_block.pop().unwrap();
                assert_eq!(x, r);
            }
        }
        decr_block.reverse();
        dec_msg.append(&mut decr_block);
    }
    dec_msg.reverse();
    dec_msg
}

