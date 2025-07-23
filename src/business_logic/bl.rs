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

use cortex_m::delay::Delay;
use hal::{
    i2c::I2c,
    prelude::*,
    serial::{Serial, config::Config},
};
use panic_halt as _;
use stm32f4xx_hal as hal;

use crate::aht20::{
    aht20_functionality::{aht20_init, aht20_measure, aht20_uart_transmit_data},
    aht20_struct::Aht20Data,
};

/// runs AHT20 sensor business logic
pub fn run_bl() {
    if let (Some(peripherals), Some(cortex_peripherals)) =
        (hal::pac::Peripherals::take(), cortex_m::Peripherals::take())
    {
        // initializes peripherals
        let gpioa = peripherals.GPIOA.split();
        let gpiob = peripherals.GPIOB.split();
        let rcc = peripherals.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(16.MHz()).freeze();

        // initializes i2c
        let scl = gpiob.pb8.into_alternate::<4>().set_open_drain();
        let sda = gpiob.pb9.into_alternate::<4>().set_open_drain();
        let mut i2c = I2c::new(peripherals.I2C1, (scl, sda), 100_000.Hz(), &clocks);

        // initializes uart
        let tx = gpioa.pa2.into_alternate::<7>();
        let rx = gpioa.pa3.into_alternate::<7>();
        let mut serial: Serial<_, u8> = Serial::new(
            peripherals.USART2,
            (tx, rx),
            Config::default().baudrate(115200.bps()),
            &clocks,
        )
        .unwrap();

        let mut sensor_data = Aht20Data::new();
        let mut delay = Delay::new(cortex_peripherals.SYST, clocks.sysclk().to_Hz());

        /*
        // AHT20 soft reset (0xBA)
        for _ in 0..3 {
            let soft_reset_cmd = [0xBA];
            let _ = i2c.write(address, &soft_reset_cmd);
            delay.delay_ms(20_u32);
        }
        */

        aht20_init(&mut sensor_data, &mut i2c, &mut serial, &mut delay);

        loop {
            aht20_measure(&mut sensor_data, &mut i2c, &mut serial, &mut delay);
            aht20_uart_transmit_data(&mut sensor_data, &mut serial);
            delay.delay_ms(1000);
        }
    }
}
