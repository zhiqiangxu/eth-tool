use hex;

pub fn remove_hex_prefix(hex_string: &str) -> &str {
    if hex_string.starts_with("0x") {
        &hex_string[2..]
    } else {
        hex_string
    }
}

pub fn decode_0xhex(hex_string: &str) -> Vec<u8> {
    return hex::decode(remove_hex_prefix(hex_string)).unwrap();
}
