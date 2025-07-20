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