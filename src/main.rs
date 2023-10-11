

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use std::time::Duration;
use anyhow::Result;
use esp_idf_hal::{spi, i2c, i2c::*, gpio::*, peripherals::Peripherals, prelude::*, spi::{Dma, SpiDriverConfig, SpiConfig}};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
};
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use weather_station::{
    config::*,
    station::*,
    wifi::*,
    display::*,
    display::display_driver::*,
};


fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let app_config = CONFIG;

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let _wifi = wifi(
        app_config.wifi_ssid,
        app_config.wifi_psk,
        peripherals.modem,
        sysloop,
        Some(nvs)
    )?;


    let display = {
        let pins = peripherals.pins;
        let spi = peripherals.spi2;
        let sclk = pins.gpio19;
        let sdo = pins.gpio23;

        let cs = pins.gpio17;
        let busy = pins.gpio18;
        let dc = pins.gpio22;
        let rst = pins.gpio21;

        let spi_driver = spi::SpiDeviceDriver::new_single(
            spi,
            sclk,
            sdo,
            Option::<AnyIOPin>::None,
            Option::<AnyOutputPin>::None,
            &SpiDriverConfig::new()
                .dma(Dma::Disabled),
            &SpiConfig::new()
                .baudrate(8.MHz().into())
        ).unwrap();

        let display_driver = DisplayDriver::new(spi_driver, DisplayPins {
            cs: PinDriver::output(cs.downgrade_output())?,
            busy: PinDriver::input(busy.downgrade_input())?,
            dc: PinDriver::output(dc.downgrade_output())?,
            rst: PinDriver::output(rst.downgrade_output())?,
        }, DisplayDriverConfig {
            delay: Duration::from_micros(200)
        });

        Display::new(display_driver, DisplayConfig {
            allow_out_of_bounds_drawing: true
        })

    }?;



    let mut weather_station = WeatherStation::new(display)?;
    weather_station.run()?;


    Ok(())
}
