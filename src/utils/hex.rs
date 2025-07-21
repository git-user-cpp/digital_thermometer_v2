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

/// Converts float to string(e.g., 12.13 -> "1213", 08.00 -> "0800")
pub fn float_to_str(num: f32) -> &'static str {
    // Multiply by 100 to shift two decimal places (e.g., 12.13 -> 1213.0)
    let scaled = (num * 100.0) as i32;

    // Handle negative numbers or out-of-range values
    if scaled < 0 || scaled > 9999 {
        return "0000"; // Fallback for invalid inputs
    }

    // Create a local buffer for digits
    let mut buffer = [b'0'; 4];
    let digits = scaled as u32;

    // Format as four digits (e.g., 1213 -> "1213")
    buffer[0] = b'0' + ((digits / 1000) % 10) as u8;
    buffer[1] = b'0' + ((digits / 100) % 10) as u8;
    buffer[2] = b'0' + ((digits / 10) % 10) as u8;
    buffer[3] = b'0' + (digits % 10) as u8;

    // Convert to &'static str using a static leak (safe in this context)
    unsafe {
        // Use a static array to store the result
        static OUTPUT: [u8; 4] = [b'0'; 4];
        // Copy to static memory (using raw pointers to avoid mutable static refs)
        let output_ptr = OUTPUT.as_ptr() as *mut u8;
        core::ptr::copy_nonoverlapping(buffer.as_ptr(), output_ptr, 4);
        core::str::from_utf8_unchecked(&OUTPUT)
    }
}
