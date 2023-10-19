use esp_idf_hal::delay::BLOCK;
use esp_idf_hal::i2c::I2cDriver;
use std::fmt::Debug;
use std::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};
use thiserror::Error;

pub struct Bme68xInterface {
    pub i2c: I2cDriver<'static>,
    pub address: u8,
}

impl Debug for Bme68xInterface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bme68xInterface")
            .field("address", &self.address)
            .finish()
    }
}

/// BME68X Errors
#[derive(Debug, Error)]
pub enum Bme68xInterfaceError {
    #[error("Null pointer")]
    NullPointer,
    #[error("Communication failure")]
    CommunicationFailure,
    #[error("I2C failure: {0:#x}")]
    I2CFailure(u8),
    #[error("Incorrect length parameter")]
    IncorrectLengthParameter,
    #[error("Device not found")]
    DeviceNotFound,
    #[error("Self test error")]
    SelfTestError,
    #[error("No new data found")]
    NoNewDataFound,
    #[error("Unstable heater")]
    UnstableHeater,
    #[error("Unknown error")]
    Unknown,
}

pub fn check_rslt(rslt: i8) -> Result<(), Bme68xInterfaceError> {
    match rslt {
        0 => Ok(()),
        -1 => Err(Bme68xInterfaceError::NullPointer),
        -2 => Err(Bme68xInterfaceError::CommunicationFailure),
        -3 => Err(Bme68xInterfaceError::DeviceNotFound),
        -4 => Err(Bme68xInterfaceError::IncorrectLengthParameter),
        -5 => Err(Bme68xInterfaceError::SelfTestError),
        2 => Err(Bme68xInterfaceError::NoNewDataFound),
        _ => Err(Bme68xInterfaceError::Unknown),
    }
}

impl Bme68xInterface {
    pub fn new(i2c: I2cDriver<'static>, address: u8) -> Self {
        Self { i2c, address }
    }

    fn read(&mut self, reg_addr: u8, reg_data: &mut [u8]) -> Result<(), Bme68xInterfaceError> {
        log::info!("Reading from register {:#x}", reg_addr);

        self.i2c
            .write_read(self.address, &[reg_addr], reg_data, BLOCK)
            .map_err(|e| {
                log::error!("I2C read error: {}", e);
                Bme68xInterfaceError::I2CFailure(reg_addr)
            })
    }

    fn write(&mut self, reg_addr: u8, reg_data: &[u8]) -> Result<(), Bme68xInterfaceError> {
        let mut buff = Vec::with_capacity(1 + reg_data.len());
        buff.push(reg_addr);
        buff.extend_from_slice(reg_data);

        log::info!("Writing {:#x?} to {:#x}", buff, reg_addr);
        self.i2c
            .write(self.address, buff.as_slice(), BLOCK)
            .map_err(|e| {
                log::error!("I2C write error: {}", e);
                Bme68xInterfaceError::I2CFailure(reg_addr)
            })
    }

    pub unsafe fn read_raw(&mut self, reg_addr: u8, reg_data: *mut u8, length: u32) -> i8 {
        let reg_slice: &mut [u8] = &mut *slice_from_raw_parts_mut(reg_data, length as usize);
        if self.read(reg_addr, reg_slice).is_ok() {
            0
        } else {
            -1
        }
    }
    pub unsafe fn write_raw(&mut self, reg_addr: u8, reg_data: *const u8, length: u32) -> i8 {
        let reg_slice: &[u8] = &*slice_from_raw_parts(reg_data, length as usize);
        if self.write(reg_addr, reg_slice).is_ok() {
            0
        } else {
            -1
        }
    }
}
