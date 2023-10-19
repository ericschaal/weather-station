use anyhow::Result;

pub struct Ambient {}

impl Ambient {
    pub fn new() -> Self {
        Ambient {}
    }

    pub fn configure_sensor(&mut self) -> Result<()> {
        Ok(())
    }
    pub fn get_readings(&mut self) -> Result<()> {
        Ok(())
    }
}
