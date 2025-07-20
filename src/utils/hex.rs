// Convert byte to hex string (e.g., 0x08 -> "08")
pub fn byte_to_hex(byte: u8) -> [u8; 2] {
    let hex_chars = b"0123456789ABCDEF";
    let high_nibble = (byte >> 4) & 0x0F;
    let low_nibble = byte & 0x0F;
    [
        hex_chars[high_nibble as usize],
        hex_chars[low_nibble as usize],
    ]
}
