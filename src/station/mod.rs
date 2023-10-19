mod ambient;
mod draw;

use crate::display::Display;
use crate::icons::WeatherIconSet;
use crate::station::ambient::Ambient;
use crate::station::draw::WeatherStationDraw;
use anyhow::Result;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use std::thread;
use std::time::Duration;

pub struct WeatherStation {
    display: Display,
    draw: WeatherStationDraw,
    ambient: Ambient,
}

pub struct Icons {
    pub large: WeatherIconSet,
    pub small: WeatherIconSet,
}

impl WeatherStation {
    pub fn new(display: Display) -> Self {
        WeatherStation {
            display,
            ambient: Ambient::new(),
            draw: WeatherStationDraw::default(),
        }
    }
    pub fn run(&mut self) -> Result<()> {
        self.ambient.configure_sensor()?;

        loop {
            //let weather = fetch_owm_report()?;
            // let sensor_data = self.ambient.get_readings()?;

            self.display.wake_up()?;
            self.display.clear(BinaryColor::Off)?;

            //self.draw.draw_weather_report(&mut self.display, weather)?;
            self.draw.draw_ambient_data(&mut self.display)?;

            self.display.flush_and_refresh()?;
            self.display.sleep()?;
            thread::sleep(Duration::from_secs(10 * 60));
        }
    }
}
