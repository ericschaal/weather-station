mod ambient;
mod current_weather;
mod draw;

use crate::bme68x::BmeDevice;
use crate::display::Display;
use crate::icons::WeatherIconSet;
use crate::owm::api::fetch_owm_report;
use crate::station::ambient::Ambient;
use crate::station::draw::WeatherStationDraw;
use anyhow::Result;
use std::thread;
use std::time::Duration;

pub struct WeatherStation<T>
where
    T: Display,
{
    display: T,
    draw: WeatherStationDraw<T>,
    ambient: Ambient,
}

pub struct Icons {
    pub large: WeatherIconSet,
    pub small: WeatherIconSet,
}

impl<T> WeatherStation<T>
where
    T: Display,
{
    pub fn new(display: T, bme: BmeDevice) -> Self {
        WeatherStation {
            display,
            ambient: Ambient::new(bme),
            draw: WeatherStationDraw::default(),
        }
    }
    pub fn run(&mut self) -> Result<()> {
        self.ambient.configure_sensor()?;

        loop {
            let weather = fetch_owm_report()?;
            // let sensor_data = self.ambient.get_readings()?;

            self.display.wake_up()?;
            self.draw.draw_weather_report(&mut self.display, weather)?;
            self.display.flush_and_refresh()?;
            self.display.sleep()?;
            thread::sleep(Duration::from_secs(10 * 60));
        }
    }
}
