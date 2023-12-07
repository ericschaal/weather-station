use crate::icons::IconSize;
use crate::owm::icons::get_icon_for_current_weather;
use crate::owm::model::CurrentWeather;
use embedded_graphics::image::Image;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::Drawable;

pub struct CurrentWeatherModule {
    viewport: Rectangle,
    current: Option<CurrentWeather>,
}

const IMG_ICON_PADDING: u32 = 16;

impl CurrentWeatherModule {
    pub fn new(viewport: Rectangle) -> Self {
        CurrentWeatherModule {
            viewport,
            current: None,
        }
    }

    pub fn update(&mut self, report: Option<CurrentWeather>) {
        self.current = report;
    }
    pub fn size() -> Size {
        let current_icon_size = Size::new(196 - IMG_ICON_PADDING, 196);
        let current_temp_size = Size::new(196, current_icon_size.height);
        let forecast_separator_size = Size::new(IMG_ICON_PADDING, current_icon_size.height);

        Size::new(
            current_temp_size.width + current_icon_size.width + forecast_separator_size.width,
            current_icon_size.height,
        )
    }
}

impl Drawable for CurrentWeatherModule {
    type Color = BinaryColor;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        if let Some(current) = &self.current {
            // current weather icon
            let icon = get_icon_for_current_weather(IconSize::Large, &current);
            Image::new(
                icon,
                self.viewport
                    .translate(Point::new(-(IMG_ICON_PADDING as i32) / 2, 0))
                    .top_left,
            )
            .draw(&mut display.color_converted())?;
        }
        Ok(())
    }
}
