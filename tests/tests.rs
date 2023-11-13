use aes_rs::*;

#[test]
fn test_conv_str() -> () {
    let text = "H1d(0sl!5;0uPcT|";
    let state = str_to_state(text);
    assert_eq!(
        Vec::from([72, 49, 100, 40, 48, 115, 108, 33, 53, 59, 48, 117, 80, 99, 84, 124]),
        state
    );
    assert_eq!(text, state_to_string(&state));
}

#[test]
fn test_add_round_key() -> () {
    let s = "[R0undKeyTest42)";

    let mut state = str_to_state(&s);

    let round_key = Vec::from([
        64, 192, 68, 82, 232, 72, 84, 19, 32, 119, 53, 8, 253, 48, 178, 89,
    ]);
    add_round_key(&mut state, &round_key);
    assert_eq!(
        Vec::from([27, 146, 116, 39, 134, 44, 31, 118, 89, 35, 80, 123, 137, 4, 128, 112]),
        state
    );
    add_round_key(&mut state, &round_key);
    assert_eq!("[R0undKeyTest42)", &state_to_string(&state));
}

#[test]
fn test_sub_bytes() -> () {
    let s = "}5ubBytesTest03]";
    let mut state = str_to_state(&s);

    sub_bytes(&mut state, false);
    assert_eq!(
        Vec::from([255, 150, 157, 170, 44, 182, 146, 77, 143, 32, 77, 143, 146, 4, 195, 76]),
        state
    );
    sub_bytes(&mut state, true);
    assert_eq!(s, state_to_string(&state));
}

#[test]
fn test_shift_rows() -> () {
    let s = "}5h1ftR0w5Test0]";
    let mut state = str_to_state(s);
    
    shift_rows(&mut state);
    assert_eq!(
        Vec::from([125, 116, 84, 93, 102, 53, 48, 49, 119, 116, 104, 48, 115, 53, 82, 101]),
        state
    );
    inv_shift_rows(&mut state);
    
    assert_eq!(s, state_to_string(&state));
}

#[test]
fn test_mix_columns() -> () {
    let s = "}M1xC0lumn5Test]";
    let mut state = str_to_state(s);
    mix_columns(&mut state, false);
    assert_eq!(
        Vec::from([100, 204, 218, 11, 207, 226, 52, 115, 9, 186, 149, 68, 118, 66, 25, 18]),
        state
    );
    mix_columns(&mut state, true);

    assert_eq!(s, state_to_string(&state));
}

#[test]
fn test_aes_block() -> () {
    let exp_key = Vec::from([
        Vec::from([
            195, 44, 92, 166, 181, 128, 94, 12, 219, 141, 165, 122, 42, 182, 254, 92,
        ]),
        Vec::from(*b"\x8c\x97\x16C9\x17HO\xe2\x9a\xed5\xc8,\x13i"),
        Vec::from(*b"\xff\xea\xef\xab\xc6\xfd\xa7\xe4$gJ\xd1\xecKY\xb8"),
        Vec::from(*b"H!\x83e\x8e\xdc$\x81\xaa\xbbnPF\xf07\xe8"),
        Vec::from(*b"\xcc\xbb\x18?Bg<\xbe\xe8\xdcR\xee\xae,e\x06"),
        Vec::from(*b"\xad\xf6w\xdb\xef\x91Ke\x07M\x19\x8b\xa9a|\x8d"),
        Vec::from(*b"b\xe6*\x08\x8dwam\x8a:x\xe6#[\x04k"),
        Vec::from(*b"\x1b\x14U.\x96c4C\x1cYL\xa5?\x02H\xce"),
        Vec::from(*b"\xecF\xde[z%\xea\x18f|\xa6\xbdY~\xees"),
        Vec::from(*b"\x04nQ\x90~K\xbb\x88\x187\x1d5AI\xf3F"),
        Vec::from(*b"\tc\x0b\x13w(\xb0\x9bo\x1f\xad\xae.V^\xe8"),
    ]);

    let s = "H1d(0sl!5;0uPcT|";
    let block = str_to_state(s);

    let encr = aes_encrypt_block(&block, &exp_key);
    //println!("{:?}", encr);
    assert_eq!(
        Vec::from([111, 190, 111, 164, 48, 24, 71, 249, 204, 179, 198, 124, 43, 248, 40, 124]),
        encr
    );
    let decr = aes_decrypt_block(&encr, &exp_key);
    assert_eq!(s, state_to_string(&decr));
}