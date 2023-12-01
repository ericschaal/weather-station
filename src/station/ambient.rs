use crate::bme68x::BmeDevice;
use anyhow::Result;

pub struct Ambient {
    bme: BmeDevice,
}

impl Ambient {
    pub fn new(bme: BmeDevice) -> Self {
        Ambient { bme }
    }

    pub fn configure_sensor(&mut self) -> Result<()> {
        Ok(())
    }
    pub fn get_readings(&mut self) -> Result<()> {
        Ok(())
    }
}
