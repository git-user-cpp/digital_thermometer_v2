/*
 * stm32f446ret_blinky
 * sipmle diode blink for stm32f446ret written in Rust 
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

#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m::peripheral::{syst::SystClkSource, Peripherals as CortexPeripherals};
use cortex_m_rt as rt;
use stm32f4xx_hal as hal;
use hal::prelude::*;

#[rt::entry]
fn main() -> ! {
    if let (Some(peripherals), Some(cortex_peripherals)) = (hal::pac::Peripherals::take(), CortexPeripherals::take()) {
        let gpioa = peripherals.GPIOA.split();
        let mut led = gpioa.pa5.into_push_pull_output();

        let mut systick = cortex_peripherals.SYST;
        systick.set_clock_source(SystClkSource::Core);
        systick.set_reload(16_000_000 - 1); // 16 MHz = 1 second
        systick.clear_current();
        systick.enable_counter();

        loop {
            led.toggle(); // Flip LED state
            while !systick.has_wrapped() {} // Wait for 1 second
        }
    }
    loop {}
}