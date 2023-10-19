use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use anyhow::Result;
use esp_idf_hal::{
    gpio::*,
    i2c::*,
    peripherals::Peripherals,
    prelude::*,
    spi,
    spi::{Dma, SpiConfig, SpiDriverConfig},
};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use weather_station::bme68x::BmeDevice;
use weather_station::{config::*, display::display_driver::*, display::*, station::*, wifi::*};

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

    // let _wifi = wifi(
    //     app_config.wifi_ssid,
    //     app_config.wifi_psk,
    //     peripherals.modem,
    //     sysloop,
    //     Some(nvs)
    // )?;

    let pins = peripherals.pins;

    let mut bme = {
        let i2c = peripherals.i2c0;
        let sda = pins.gpio26;
        let scl = pins.gpio27;

        let i2c_driver =
            I2cDriver::new(i2c, sda, scl, &I2cConfig::new().baudrate(100.kHz().into())).unwrap();

        BmeDevice::new(i2c_driver)
    }?;
    //
    // bme.init()?;
    log::info!("BME68x initialized");
    bme.self_test()?;
    log::info!("BME68x self test passed");

    // let display = {
    //     let spi = peripherals.spi2;
    //
    //     let sclk = pins.gpio19;
    //     let sdo = pins.gpio23;
    //
    //     let cs = pins.gpio17;
    //     let busy = pins.gpio18;
    //     let dc = pins.gpio22;
    //     let rst = pins.gpio21;
    //
    //     let spi_driver = spi::SpiDeviceDriver::new_single(
    //         spi,
    //         sclk,
    //         sdo,
    //         Option::<AnyIOPin>::None,
    //         Option::<AnyOutputPin>::None,
    //         &SpiDriverConfig::new()
    //             .dma(Dma::Disabled),
    //         &SpiConfig::new()
    //             .baudrate(8.MHz().into())
    //     ).unwrap();
    //
    //     let display_driver = DisplayDriver::new(spi_driver, DisplayPins {
    //         cs: PinDriver::output(cs.downgrade_output())?,
    //         busy: PinDriver::input(busy.downgrade_input())?,
    //         dc: PinDriver::output(dc.downgrade_output())?,
    //         rst: PinDriver::output(rst.downgrade_output())?,
    //     }, DisplayDriverConfig {
    //         delay: Duration::from_micros(200)
    //     });
    //
    //     Display::new(display_driver, DisplayConfig {
    //         allow_out_of_bounds_drawing: true
    //     })
    //
    // }?;

    // let mut weather_station = WeatherStation::new(display, bme);
    // weather_station.run()?;

    Ok(())
}
