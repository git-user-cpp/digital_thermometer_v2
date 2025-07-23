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

/// Holds data for AHT20 sensor
pub struct Aht20Data {
    pub device_address: u8,
    pub measured_data: [u8; 7],
    pub humidity: f32,
    pub temp_c: f32,
    pub temp_f: f32,
}

impl Aht20Data {
    /// Creates new instance and initializes it's device address
    pub fn new() -> Self {
        Self {
            device_address: 0x38,
            measured_data: [0, 0, 0, 0, 0, 0, 0],
            humidity: 0.0,
            temp_c: 0.0,
            temp_f: 0.0,
        }
    }
}
