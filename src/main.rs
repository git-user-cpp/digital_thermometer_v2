/*
 * digital thermomether
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

#![no_main]
#![no_std]

use core::fmt::Write;
use cortex_m::delay::Delay;
use cortex_m_rt as rt;
use hal::{
    i2c::I2c,
    prelude::*,
    serial::{Serial, config::Config},
};
use panic_halt as _;
use stm32f4xx_hal as hal;

// Convert byte to hex string (e.g., 0x08 -> "08")
fn byte_to_hex(byte: u8) -> [u8; 2] {
    let hex_chars = b"0123456789ABCDEF";
    let high_nibble = (byte >> 4) & 0x0F;
    let low_nibble = byte & 0x0F;
    [
        hex_chars[high_nibble as usize],
        hex_chars[low_nibble as usize],
    ]
}

#[rt::entry]
fn main() -> ! {
    if let (Some(peripherals), Some(cortex_peripherals)) =
        (hal::pac::Peripherals::take(), cortex_m::Peripherals::take())
    {
        // initializing peripherals
        let gpioa = peripherals.GPIOA.split();
        let gpiob = peripherals.GPIOB.split();
        let rcc = peripherals.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(16.MHz()).freeze();

        let mut delay = Delay::new(cortex_peripherals.SYST, clocks.sysclk().to_Hz());
        // delay after power on due to AHT20 datasheet
        delay.delay_ms(40_u32);

        // Initializing pins for I2C connection
        let scl = gpiob.pb8.into_alternate::<4>().set_open_drain();
        let sda = gpiob.pb9.into_alternate::<4>().set_open_drain();

        // initializing i2c connection
        let mut i2c = I2c::new(peripherals.I2C1, (scl, sda), 100_000.Hz(), &clocks);

        let tx = gpioa.pa2.into_alternate::<7>();
        let rx = gpioa.pa3.into_alternate::<7>();
        let mut serial: Serial<_, u8> = Serial::new(
            peripherals.USART2,
            (tx, rx),
            Config::default().baudrate(115200.bps()),
            &clocks,
        )
        .unwrap();

        // i2c device address
        let address = 0x38;

        /*
        // AHT20 soft reset (0xBA)
        for _ in 0..3 {
            let soft_reset_cmd = [0xBA];
            let _ = i2c.write(address, &soft_reset_cmd);
            delay.delay_ms(20_u32);
        }
        */

        /*
        // AHT20 initialization command
        let init_cmd = [0xBE, 0x08, 0x00];
        match i2c.write(address, &init_cmd) {
            Ok(_) => {
                let msg = "AHT20 Init OK\r\n";
                serial.write_str(msg);
            }
            Err(_e) => {
                let msg = "AHT20 Init NACK\r\n";
                serial.write_str(msg);
            }
        }
        delay.delay_ms(50_u32); // Extended initialization delay
        */

        // Check calibration status
        let check_calibration_cmd = [0x71];
        let mut cal_status = [0u8; 1];
        match i2c.write_read(address, &check_calibration_cmd, &mut cal_status) {
            Ok(_) => {
                let msg = "Calibration Status: 0x";
                let _ = serial.write_str(msg);
                delay.delay_ms(10_u32);
                
                let hex = byte_to_hex(cal_status[0]);
                let _ = serial.write(hex[0]);
                delay.delay_ms(10_u32);
                let _ = serial.write(hex[1]);
                delay.delay_ms(10_u32);
                let _ = serial.write_str("\r\n");
                delay.delay_ms(10_u32);
            }
            Err(_e) => {
                let msg = "Calibration Check NACK\r\n";
                let _ = serial.write_str(msg);
            }
        }
        delay.delay_ms(500_u32); // Delay between attempts

        loop {
            // match i2c.read(address, &mut buffer) {
            //     Ok(_) => {
            //         // Send data over UART
            //         for &byte in buffer.iter() {
            //             let _ = serial.write(byte);
            //         }
            //     }
            //     Err(_e) => {
            //         // Handle I2C read error (e.g., send error message over UART)
            //         let _ = serial.write_str("I2C Error\n");
            //     }
            // }
        }
    }
    loop {}
}
