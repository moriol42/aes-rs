use aes_rs::*;

fn main() {
    /*let mut state : Vec<u8> = Vec::from([
        206, 243, 61, 34,
        171, 11, 93, 31,
        16, 200, 91, 108,
        150, 3, 194, 51
    ]);

    let round_key : Vec<u8> = Vec::from([
        173, 129, 68, 82,
        223, 100, 38, 109,
        32, 189, 53, 8,
        253, 48, 187, 78,
    ]);
    //println!("{}", state_to_string(&state));
    add_round_key(&mut state, &round_key);
    println!("{:?}", state);
    println!("{}", state_to_string(&state));
    add_round_key(&mut state, &round_key);
    //println!("{}", state_to_string(&state));*/

    /*let text = "abcdefghijklmnop";
    let state = str_to_state(text);
    println!("{:?}", state);
    let s = state_to_string(&state);

    println!("{}", s);

    let mut state = Vec::from([
        251, 64, 182, 81,
        146, 168, 33, 80,
        199, 159, 195, 24,
        64, 80, 182, 255,
    ]);

    sub_bytes(&mut state, true);
    println!("{}", state_to_string(&state));*/

    /*let mut state = Vec::from([
        108, 106, 71, 86,
        96, 62, 38, 72,
        42, 184, 92, 209,
        94, 79, 8, 54,
    ]);
    
    mix_columns(&mut state, true);
    println!("{:?}", state);
    inv_shift_rows(&mut state);

    println!("{}", state_to_string(&state));*/

    let exp_key = Vec::from([ 
        Vec::from([195, 44, 92, 166, 181, 128, 94, 12, 219, 141, 165, 122, 42, 182, 254, 92]),
        Vec::from(*b"\x8c\x97\x16C9\x17HO\xe2\x9a\xed5\xc8,\x13i"), 
        Vec::from(*b"\xff\xea\xef\xab\xc6\xfd\xa7\xe4$gJ\xd1\xecKY\xb8"), 
        Vec::from(*b"H!\x83e\x8e\xdc$\x81\xaa\xbbnPF\xf07\xe8"), 
        Vec::from(*b"\xcc\xbb\x18?Bg<\xbe\xe8\xdcR\xee\xae,e\x06"), 
        Vec::from(*b"\xad\xf6w\xdb\xef\x91Ke\x07M\x19\x8b\xa9a|\x8d"), 
        Vec::from(*b"b\xe6*\x08\x8dwam\x8a:x\xe6#[\x04k"), 
        Vec::from(*b"\x1b\x14U.\x96c4C\x1cYL\xa5?\x02H\xce"), 
        Vec::from(*b"\xecF\xde[z%\xea\x18f|\xa6\xbdY~\xees"), 
        Vec::from(*b"\x04nQ\x90~K\xbb\x88\x187\x1d5AI\xf3F"), 
        Vec::from(*b"\tc\x0b\x13w(\xb0\x9bo\x1f\xad\xae.V^\xe8")]);

    let block = str_to_state("abcdefghijklmnop");

    let encr = aes_encrypt_block(&block, &exp_key);
    println!("{:?}", encr);
    let decr = aes_decrypt_block(&encr, &exp_key);
    println!("{:?}", decr);

    let block = Vec::from(*b"\xd1O\x14j\xa4+O\xb6\xa1\xc4\x08B)\x8f\x12\xdd");
    let decr = aes_decrypt_block(&block, &exp_key);
    println!("{}", state_to_string(&decr));


}
