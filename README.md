# digital_thermometer_v2
digital thermomether for stm32f446ret written in Rust

---

## How to use

1. cargo build --release --target thumbv7em-none-eabihf
2. arm-none-eabi-objcopy -O ihex target/thumbv7em-none-eabihf/release/hum_temp_sensor target/thumbv7em-none-eabihf/release/hum_temp_sensor.hex
3. flash the device using stmCubeProgrammer and created .hex file
4. press reset button. done.

---

## ⚠️ LICENSE ⚠️

    stm32f446ret_blinky
    sipmle diode blink for stm32f446ret written in Rust
    Copyright (C) 2025  Andrew Kushyk
   
    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published
    by the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.
   
    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.
   
    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.

---
