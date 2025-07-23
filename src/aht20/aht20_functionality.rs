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

use core::fmt::Write;
use cortex_m::delay::Delay;
use hal::{
    i2c::I2c,
    pac::{I2C1, USART2},
    serial::Serial,
};
use panic_halt as _;
use stm32f4xx_hal::{self as hal, uart::TxISR};

use crate::aht20::aht20_struct::Aht20Data;
use crate::{aht20::aht20_commands::Aht20Commands, utils::convert::float_to_uart};

/// Initializes AHT20 sensor
pub fn aht20_init(
    mut sensor_data: &mut Aht20Data,
    mut i2c: &mut I2c<I2C1>,
    mut serial: &mut Serial<USART2, u8>,
    mut delay: &mut Delay,
) {
    let mut calibration_status = [0u8; 1];

    // delay after power on due to AHT20 datasheet
    delay.delay_ms(40_u32);

    match i2c.write_read(
        sensor_data.device_address,
        &Aht20Commands::CHECK_CALIBRATION.as_bytes(),
        &mut calibration_status,
    ) {
        Ok(_) => {
            // calibrates the sensor if wasn't calibrated
            if (calibration_status[0] & (1 << 3)) == 0 {
                aht20_calibrate(&mut sensor_data, &mut i2c, &mut serial, &mut delay);
            }
        }
        Err(_e) => {
            let msg = "Calibration Check NACK\r\n";
            let _ = serial.write_str(msg);
        }
    }
}

/// Triggers sensor measurment
pub fn aht20_measure(
    mut sensor_data: &mut Aht20Data,
    i2c: &mut I2c<I2C1>,
    serial: &mut Serial<USART2, u8>,
    delay: &mut Delay,
) {
    match i2c.write(
        sensor_data.device_address,
        &Aht20Commands::MEASURE.as_bytes(),
    ) {
        Ok(_) => {
            delay.delay_ms(80);

            match i2c.read(sensor_data.device_address, &mut sensor_data.measured_data) {
                Ok(_) => {
                    if sensor_data.measured_data[0] & 0x80 == 0 {
                        if aht20_check_crc(sensor_data) == sensor_data.measured_data[6] {
                            aht20_calculate_measurments(&mut sensor_data);
                        } else {
                            let msg = "AHT20 CRC check failed\r\n";
                            let _ = serial.write_str(msg);
                        }
                    } else {
                        let msg = "AHT20 Busy\r\n";
                        let _ = serial.write_str(msg);
                    };
                }
                Err(_e) => {
                    aht20_soft_reset(i2c, sensor_data, delay);
                    let msg = "AHT20 Failed to get data. Performing soft reset...\r\n";
                    let _ = serial.write_str(msg);
                }
            }
        }
        Err(_e) => {
            let msg = "AHT20 Failed measurment\r\n";
            let _ = serial.write_str(msg);
        }
    }
}

/// Transmits measured data via UART
pub fn aht20_uart_transmit_data(sensor_data: &mut Aht20Data, serial: &mut Serial<USART2, u8>) {
    let parts = [
        ("Humidity: ", float_to_uart(sensor_data.humidity)),
        (", C: ", float_to_uart(sensor_data.temp_c)),
        (", F: ", float_to_uart(sensor_data.temp_f)),
        ("\r\n", [0; 6]),
    ];
    for (prefix, data) in parts.iter() {
        let _ = serial.write_str(prefix);
        for &byte in data.iter().filter(|&&b| b != 0) {
            while !serial.is_tx_empty() {} // Wait for buffer to be ready
            let _ = serial.write_char(byte as char);
        }
    }
}

// Helper function for sensor calibration
fn aht20_calibrate(
    sensor_data: &mut Aht20Data,
    i2c: &mut I2c<I2C1>,
    serial: &mut Serial<USART2, u8>,
    delay: &mut Delay,
) {
    match i2c.write(
        sensor_data.device_address,
        &Aht20Commands::CALIBRATE.as_bytes(),
    ) {
        Ok(_) => {
            let msg = "AHT20 Init OK\r\n";
            let _ = serial.write_str(msg);
        }
        Err(_e) => {
            let msg = "AHT20 Init NACK\r\n";
            let _ = serial.write_str(msg);
        }
    }
    delay.delay_ms(10_u32); // Extended initialization delay
}

// Helper function for calculating measurments
fn aht20_calculate_measurments(sensor_data: &mut Aht20Data) {
    let raw_humidity: u32 = ((sensor_data.measured_data[1] as u32) << 12)
        | ((sensor_data.measured_data[2] as u32) << 4)
        | ((sensor_data.measured_data[3] as u32) >> 4);
    let raw_temperature: u32 = (((sensor_data.measured_data[3] as u32) & 0x0F) << 16)
        | ((sensor_data.measured_data[4] as u32) << 8)
        | (sensor_data.measured_data[5] as u32);

    sensor_data.humidity = (raw_humidity as f32 * 100.0) / 1048576.0;
    sensor_data.temp_c = (raw_temperature as f32 * 200.0 / 1048576.0) - 50.0;
    sensor_data.temp_f = sensor_data.temp_c * 9.0 / 5.0 + 32.0;
}

// Helper function for sensor soft reset
fn aht20_soft_reset(i2c: &mut I2c<I2C1>, sensor_data: &mut Aht20Data, delay: &mut Delay) {
    let soft_reset_cmd = [0xBA];
    let _ = i2c.write(sensor_data.device_address, &soft_reset_cmd);
    delay.delay_ms(20_u32);
}

// Helper function for checking crc
fn aht20_check_crc(sensor_data: &mut Aht20Data) -> u8 {
    let mut crc: u8 = 0xFF;

    for &byte in sensor_data.measured_data.iter().take(6) {
        crc ^= byte;
        for _ in 0..8 {
            if crc & 0x80 != 0 {
                crc = (crc << 1) ^ 0x31;
            } else {
                crc <<= 1;
            }
        }
    }

    crc
}
