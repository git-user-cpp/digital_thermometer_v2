/*
 * digital_thermometer_v2
 * digital thermomether for stm32f446ret written in Rust
 * Copyright (C) 2025  Andrew Kushyk
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

/// Converts byte to hex string (e.g., 0x08 -> "08")
pub fn byte_to_hex(byte: u8) -> [u8; 2] {
    let hex_chars = b"0123456789ABCDEF";
    let high_nibble = (byte >> 4) & 0x0F;
    let low_nibble = byte & 0x0F;
    [
        hex_chars[high_nibble as usize],
        hex_chars[low_nibble as usize],
    ]
}

/// Converts float to string (e.g., 12.13 -> "12.13", 08.00 -> "08.00")
pub fn float_to_str(num: f32) -> [u8; 5] {
    let dec_chars = b"0123456789.";
    let scaled = (num * 100.0) as u32;
    let tens = (scaled / 1000) % 10;
    let ones = (scaled / 100) % 10;
    let dec1 = (scaled / 10) % 10;
    let dec2 = scaled % 10;
    let period: u8 = 10;

    [
        dec_chars[tens as usize],
        dec_chars[ones as usize],
        dec_chars[period as usize],
        dec_chars[dec1 as usize],
        dec_chars[dec2 as usize],
    ]
}
