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

/// Holds essential AHT20 commands
pub enum Aht20Commands {
    CheckCalibration([u8; 1]),
    Calibrate([u8; 3]),
    Measure([u8; 3]),
}

impl Aht20Commands {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Aht20Commands::CheckCalibration(bytes) => bytes,
            Aht20Commands::Calibrate(bytes) => bytes,
            Aht20Commands::Measure(bytes) => bytes,
        }
    }

    pub const CHECK_CALIBRATION: Self = Aht20Commands::CheckCalibration([0x71]);
    pub const CALIBRATE: Self = Aht20Commands::Calibrate([0xBE, 0x08, 0x00]);
    pub const MEASURE: Self = Aht20Commands::Measure([0xAC, 0x33, 0x00]);
}
