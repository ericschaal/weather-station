mod constants;
mod device;
mod driver;
mod interface;
mod wrapper;

use crate::bme68x::constants::BME68X_I2C_ADDR_LOW;
use crate::bme68x::driver::{bme68x_dev, bme68x_init, bme68x_selftest_check};
use crate::bme68x::interface::{check_rslt, Bme68xInterface, Bme68xInterfaceError};
use anyhow::Result;
use esp_idf_hal::i2c::I2cDriver;

pub struct BmeDevice {
    interface: Bme68xInterface,
    device: bme68x_dev,
}

impl BmeDevice {
    pub fn new(i2c: I2cDriver<'static>) -> Result<Self, Bme68xInterfaceError> {
        let interface = Bme68xInterface::new(i2c, BME68X_I2C_ADDR_LOW);

        let device = bme68x_dev::default();

        Ok(Self { interface, device })
    }

    pub fn init(&mut self) -> Result<(), Bme68xInterfaceError> {
        self.device.intf_ptr =
            &mut self.interface as *mut Bme68xInterface as *mut ::std::os::raw::c_void;
        let rslt = unsafe { bme68x_init(&mut self.device) };
        check_rslt(rslt)
    }

    pub fn self_test(&mut self) -> Result<(), Bme68xInterfaceError> {
        self.device.intf_ptr =
            &mut self.interface as *mut Bme68xInterface as *mut ::std::os::raw::c_void;
        let rst = unsafe { bme68x_selftest_check(&self.device) };
        check_rslt(rst)
    }
}
