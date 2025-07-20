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
