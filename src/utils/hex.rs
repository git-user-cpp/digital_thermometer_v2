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

/// Converts byte to decimal string (e.g., 123 -> "123", 8 -> "8")
pub fn float_to_str(num: f32) -> &'static str {
    todo!()
}
