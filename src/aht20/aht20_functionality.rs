use core::fmt::Write;
use cortex_m::delay::Delay;
use hal::{
    i2c::I2c,
    pac::{I2C1, USART2},
    prelude::*,
    serial::{Serial},
};
use panic_halt as _;
use stm32f4xx_hal as hal;

use crate::aht20::aht20_struct::Aht20Data;
use crate::aht20::aht20_commands::Aht20Commands;
use crate::utils::hex::byte_to_hex;

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
            let msg = "Calibration Status: 0x";
            let _ = serial.write_str(msg);
            delay.delay_ms(10_u32);

            let hex = byte_to_hex(calibration_status[0]);
            let _ = serial.write(hex[0]);
            delay.delay_ms(10_u32);
            let _ = serial.write(hex[1]);
            delay.delay_ms(10_u32);
            let _ = serial.write_str("\r\n");
            delay.delay_ms(10_u32);

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

fn aht20_calibrate(
    sensor_data: &mut Aht20Data,
    i2c: &mut I2c<I2C1>,
    serial: &mut Serial<USART2, u8>,
    delay: &mut Delay,
) {
    match i2c.write(sensor_data.device_address, &Aht20Commands::CALIBRATE.as_bytes()) {
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

pub fn aht20_measure(sensor_data: &mut Aht20Data, i2c: &mut I2c<I2C1>, serial: &mut Serial<USART2, u8>, delay: &mut Delay) {
    match i2c.write(sensor_data.device_address, &Aht20Commands::MEASURE.as_bytes()) {
        Ok(_) => {
            delay.delay_ms(80);

            match i2c.read(sensor_data.device_address, &mut sensor_data.measured_data) {
                Ok(_) => {
                    let msg = "AHT20 Data:\r\n";
                    let _ = serial.write_str(msg);
                    delay.delay_ms(10_u32);

                    for (i, &byte) in sensor_data.measured_data.iter().enumerate() {
                        let index_msg = ['0' as u8 + i as u8];
                        let _ = serial.write_str("Byte ");
                        delay.delay_ms(10_u32);
                        let _ = serial.write(index_msg[0]);
                        delay.delay_ms(10_u32);
                        let _ = serial.write_str(": 0x");
                        delay.delay_ms(10_u32);

                        let hex = byte_to_hex(byte);
                        let _ = serial.write(hex[0]);
                        delay.delay_ms(10_u32);
                        let _ = serial.write(hex[1]);
                        delay.delay_ms(10_u32);

                        let _ = serial.write_str("\r\n");
                        delay.delay_ms(10_u32);
                    }

                    let msg = "Data read complete\r\n";
                    let _ = serial.write_str(msg);
                }
                Err(_e) => {
                    let msg = "AHT20 Failed to get data\r\n";
                    let _ = serial.write_str(msg);
                }
            }
        }
        Err(_e) => {
            let msg = "AHT20 Failed to measure\r\n";
            let _ = serial.write_str(msg);
        }
    }
}
