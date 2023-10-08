use std::time::Duration;
use anyhow::Result;
use esp_idf_hal::{gpio::*, delay::*, spi::*, delay};
use esp_idf_sys::EspError;
use log::*;
use crate::display::command::Command;
use crate::display::display::{BUFFER_SIZE};
use crate::display::traits;

pub struct DisplayDriver {
    spi: SpiDeviceDriver<'static, SpiDriver<'static>>,
    pins: DisplayPins,
    config: DisplayDriverConfig,
}

pub struct DisplayPins {
    pub cs: PinDriver<'static, AnyOutputPin, Output>,
    pub busy: PinDriver<'static, AnyInputPin, Input>,
    pub dc: PinDriver<'static, AnyOutputPin, Output>,
    pub rst: PinDriver<'static, AnyOutputPin, Output>,
}

pub struct DisplayDriverConfig {
    pub delay: Duration,
}

impl DisplayDriver
{
    pub fn new(driver: SpiDeviceDriver<'static, SpiDriver<'static>>, pins: DisplayPins, config: DisplayDriverConfig) -> DisplayDriver {
        DisplayDriver {
            spi: driver,
            pins,
            config,
        }
    }

    pub fn transmit_frame(&mut self, frame: &[u8]) -> Result<(), EspError> {
        self.wait_until_idle()?;
        info!("Transmitting frame");
        self.cmd_with_data(Command::DataStartTransmission2, frame)
    }

    pub fn refresh(&mut self) -> Result<(), EspError> {
        info!("Display refresh");
        self.cmd(Command::DisplayRefresh)?;
        Delay::delay_ms(50);

        self.wait_until_idle()
    }

    pub fn init(&mut self) -> Result<(), EspError> {
        info!("Initializing display");

        self.reset()?;
        info!("Display reset");

        self.cmd_with_data(Command::PowerSetting, &[0x07, 0x07, 0x3f, 0x3f])?;
        self.cmd(Command::PowerOn)?;
        Delay::delay_ms(100);

        self.cmd_with_data(Command::PanelSetting, &[0x1F])?;
        self.cmd_with_data(Command::TconResolution, &[0x03, 0x20, 0x01, 0xE0])?;

        self.cmd_with_data(Command::DualSpi, &[0x00])?;
        self.cmd_with_data(Command::VcomAndDataIntervalSetting, &[0x10, 0x07])?;

        self.cmd_with_data(Command::TconSetting, &[0x22])?;

        info!("Display initialized");

        Ok(())
    }

    fn wait_until_idle(&mut self) -> Result<(), EspError> {
        info!("Waiting for display to become idle");
        self.cmd(Command::GetStatus)?;

        while self.is_busy() {
            if !self.config.delay.is_zero() {
                Delay::delay_ms(self.config.delay.as_millis() as u32);
            }
        }
        info!("Display is idle");

        Ok(())
    }

    fn cmd_with_data<T: traits::Command>(&mut self, cmd: T, data: &[u8]) -> Result<(), EspError> {
        self.cmd(cmd)?;
        self.data(data)
    }

    fn cmd<T: traits::Command>(&mut self, cmd: T) -> Result<(), EspError> {
        // low for commands
        let _ = self.pins.dc.set_low()?;
        self.write(&[cmd.address()])
    }

    fn data(&mut self, data: &[u8]) -> Result<(), EspError> {
        // high for data
        let _ = self.pins.dc.set_high()?;
        self.write(data)
    }

    fn write(&mut self, data: &[u8]) -> Result<(), EspError> {
        self.pins.cs.set_low()?;
        self.spi.write(data)?;
        self.pins.cs.set_high()
    }

    fn is_busy(&self) -> bool {
        self.pins.busy.is_low()
    }

    pub fn clear_screen(&mut self) -> Result<(), EspError> {

        info!("Clearing screen");

        self.cmd_with_data(Command::DataStartTransmission1, &[0x00; BUFFER_SIZE])?;
        self.cmd_with_data(Command::DataStartTransmission2, &[0x00; BUFFER_SIZE])?;

        self.refresh()
    }

    pub fn reset(&mut self) -> Result<(), EspError> {
        let _ = self.pins.rst.set_high()?;
        Delay::delay_ms(200);

        let _ = self.pins.rst.set_low()?;
        Delay::delay_ms(2);

        let _ = self.pins.rst.set_high()?;
        Delay::delay_ms(200);

        Ok(())
    }

}