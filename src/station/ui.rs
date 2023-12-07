use crate::display::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use crate::icons::Icons;
use crate::owm::model::WeatherData;
use crate::station::current_weather::CurrentWeatherModule;
use anyhow::Result;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::Drawable;

pub struct WeatherStationUI {
    current_weather: CurrentWeatherModule,
}
const MARGIN: u32 = 8;
const IMG_ICON_PADDING: u32 = 16;

impl WeatherStationUI {
    pub fn new() -> Self {
        let viewport_size = Size::new(DISPLAY_WIDTH - MARGIN, DISPLAY_HEIGHT - MARGIN);

        let viewport = Rectangle::new(Point::new(MARGIN as i32, MARGIN as i32), viewport_size);
        let current_weather = Rectangle::new(viewport.top_left, CurrentWeatherModule::size());

        let current_weather = CurrentWeatherModule::new(current_weather);

        WeatherStationUI { current_weather }
    }
    pub fn update(&mut self, weather: &WeatherData) -> Result<()> {
        self.current_weather.update(weather.current.clone());
        Ok(())
    }
}

impl Drawable for WeatherStationUI {
    type Color = BinaryColor;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        self.current_weather.draw(display)?;
        Ok(())
    }
}
