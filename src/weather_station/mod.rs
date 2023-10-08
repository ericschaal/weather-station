use std::thread;
use std::time::Duration;
use anyhow::Result;
use embedded_graphics::{
    primitives::*,
    geometry::*,
    prelude::*,
    image::*
};
use embedded_graphics::pixelcolor::BinaryColor;
use u8g2_fonts::{
    FontRenderer,
    types::*,
    fonts,
};
use crate::config::CONFIG;
use crate::display::display::{Display, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use crate::icons::{
    WeatherIconSet,
};
use crate::owm::{
    api::fetch_owm_report,
    model::{CurrentWeather,DailyForecast, HourlyForecast, WeatherData},
    icons::get_icon_for_current_weather,
};
use crate::owm::icons::get_icon_for_daily_forecast;

const MARGIN: u32 = 8;
// Seems like icons have a bunch of padding on the horizontal axis
// This is a dirty attempt to gain some screen space
// Padding is actually closer to 32, but this is enough to make things fit
const IMG_ICON_PADDING: u32 = 16;

pub struct WeatherStation {
    display: Display,
    rect: DisplayRect,
}

pub struct DisplayRect {
    pub viewport: Rectangle,
    pub current_weather: Rectangle,
    pub weather_icon: Rectangle,
    pub current_temp: Rectangle,
    pub feels_like: Rectangle,
    pub current_temp_unit: Rectangle,
    pub date_location: Rectangle,
    pub forecast: Rectangle,
    pub forecasts: Vec<Rectangle>,
    pub metrics: Rectangle,
    pub chart: Rectangle,
}

impl WeatherStation {
    pub fn new(display: Display) -> Self {

        let viewport_size = Size::new(DISPLAY_WIDTH - MARGIN, DISPLAY_HEIGHT - MARGIN);
        let current_icon_size = Size::new(196 - IMG_ICON_PADDING, 196);
        let current_temp_size = Size::new(196, current_icon_size.height);

        let forecast_separator_size = Size::new(IMG_ICON_PADDING, current_icon_size.height);

        let current_weather_size = Size::new(current_temp_size.width + current_icon_size.width + forecast_separator_size.width, current_icon_size.height);

        let temp_unit_size = Size::new(32, current_temp_size.height);
        let temp_feels_like_size = Size::new(current_temp_size.width, 32);

        let date_location_size = Size::new(viewport_size.width - current_weather_size.width, 64);
        let forecast_size = Size::new(viewport_size.width - current_weather_size.width, current_weather_size.height - date_location_size.height);

        let metrics_size = Size::new(294, viewport_size.height - current_weather_size.height);
        let chart_size = Size::new(viewport_size.width - metrics_size.width, viewport_size.height - current_weather_size.height);

        let viewport = Rectangle::new(Point::new(MARGIN as i32, MARGIN as i32), viewport_size);
        let current_weather = Rectangle::new(viewport.top_left, current_weather_size);
        let weather_icon = Rectangle::new(current_weather.top_left, current_icon_size);

        let current_temp = Rectangle::new(weather_icon.anchor_point(AnchorPoint::TopRight), current_temp_size);
        let forecast_separator = Rectangle::new(current_temp.anchor_point(AnchorPoint::TopRight), forecast_separator_size);

        let current_temp_unit = Rectangle::new(current_temp.anchor_point(AnchorPoint::TopRight), temp_unit_size)
            .translate(Point::new(-(temp_unit_size.width as i32), 0));

        let feels_like = Rectangle::new(current_temp.anchor_point(AnchorPoint::BottomLeft), temp_feels_like_size)
            .translate(Point::new(0, -(temp_feels_like_size.height as i32)));

        let date_location = Rectangle::new(forecast_separator.anchor_point(AnchorPoint::TopRight), date_location_size);

        let forecast = Rectangle::new(date_location.anchor_point(AnchorPoint::BottomLeft), forecast_size);
        let forecasts = [0; 5].iter().enumerate().map(|(i, _)| {
            let size = Size::new(forecast_size.width / 5, forecast_size.height);
            let offset_x = size.width * i as u32;
            Rectangle::new(forecast.anchor_point(AnchorPoint::TopLeft) + Point::new(offset_x as i32, 0), size)
        }).collect::<Vec<_>>();

        let metrics = Rectangle::new(current_weather.anchor_point(AnchorPoint::BottomLeft), metrics_size);
        let chart = Rectangle::new(metrics.anchor_point(AnchorPoint::TopRight), chart_size);

        let rect = DisplayRect {
            viewport,
            current_weather,
            weather_icon,
            current_temp,
            feels_like,
            current_temp_unit,
            date_location,
            forecast,
            forecasts,
            metrics,
            chart,
        };

        WeatherStation {
            display,
            rect
        }
    }
    pub fn run(&mut self) -> Result<()> {
        loop {
            let weather = fetch_owm_report()?;

            self.display.clear(BinaryColor::Off)?;
            self.draw_weather_report(weather)?;

            self.display.flush_and_refresh()?;

            thread::sleep(Duration::from_secs(60));
        }
    }

    fn draw_weather_report(&mut self, weather: WeatherData) -> Result<()> {
        let app_config = CONFIG;
        let location_name = app_config.location_name;
        let current = weather.current.unwrap();
        let daily = weather.daily.unwrap();
        let hourly = weather.hourly.unwrap();
        let dt = current.dt;

        let large_icon_set = WeatherIconSet::new()?;
        let small_icon_set = WeatherIconSet::new_small()?;

        self.current_weather_icon(&large_icon_set, &current)?;
        self.current_temperature(&current)?;
        self.current_feels_like(&current)?;
        self.current_temp_unit()?;
        self.date_and_location(dt, location_name)?;
        self.daily_forecast(&small_icon_set, &daily)?;

        self.debug_draw_rect()?;


        Ok(())
    }

    fn current_weather_icon(&mut self, icons: &WeatherIconSet, current: &CurrentWeather) -> Result<()> {
        let icon = get_icon_for_current_weather(icons, current);
        Image::new(icon, self.rect.current_weather
            .translate(Point::new(-(IMG_ICON_PADDING as i32) / 2, 0)).top_left)
            .draw(&mut self.display.color_converted())?;
        Ok(())
    }

    fn current_temp_unit(&mut self) -> Result<()> {
        let unit_style = PrimitiveStyleBuilder::new()
            .stroke_width(4)
            .stroke_color(BinaryColor::On)
            .build();

        let circle_diameter: u32 = 12;
        let circle_center = Point::new(circle_diameter as i32 / 2, circle_diameter as i32 / 2);
        let offset = Point::new(0, 46); // 46 is half the font size
        Circle::new(
            self.rect.current_temp_unit.center() - offset - circle_center,
            circle_diameter
        ).into_styled(unit_style)
            .draw(&mut self.display)?;

        Ok(())
    }

    fn current_feels_like(&mut self, current: &CurrentWeather) -> Result<()> {
        let font = FontRenderer::new::<fonts::u8g2_font_profont22_tf>();

        font.render_aligned(
            format_args!("Feels Like {}°", current.feels_like.round() as i32),
            self.rect.feels_like.bounding_box().center(),
            VerticalPosition::Center,
            HorizontalAlignment::Center,
            FontColor::Transparent(BinaryColor::On),
            &mut self.display,
        ).unwrap();

        Ok(())
    }

    fn current_temperature(&mut self, current: &CurrentWeather) -> Result<()> {
        let large_font = FontRenderer::new::<fonts::u8g2_font_logisoso92_tn>();

        large_font.render_aligned(
            format_args!("{}", current.temp.round() as i32),
            self.rect.current_temp.bounding_box().center(),
            VerticalPosition::Center,
            HorizontalAlignment::Center,
            FontColor::Transparent(BinaryColor::On),
            &mut self.display,
        ).unwrap();

        Ok(())
    }

    fn date_and_location(&mut self, current_time: u64, location_name: &str) -> Result<()> {
        let large = FontRenderer::new::<fonts::u8g2_font_profont29_tf>();
        let font = FontRenderer::new::<fonts::u8g2_font_profont22_tf>();

        let offset_dt = time::OffsetDateTime::from_unix_timestamp(current_time as i64)?;
        let format = time::format_description::parse("[weekday], [day] [month repr:long] [year]")?;
        let formatted = offset_dt.format(&format)?;

        large.render_aligned(
            location_name,
            self.rect.date_location.anchor_point(AnchorPoint::TopRight),
            VerticalPosition::Top,
            HorizontalAlignment::Right,
            FontColor::Transparent(BinaryColor::On),
            &mut self.display,
        ).unwrap();

        font.render_aligned(
            formatted.as_str(),
            self.rect.date_location.anchor_point(AnchorPoint::TopRight) + Point::new(0, 29),
            VerticalPosition::Top,
            HorizontalAlignment::Right,
            FontColor::Transparent(BinaryColor::On),
            &mut self.display,
        ).unwrap();

        Ok(())
    }

    fn daily_forecast(&mut self, icons: &WeatherIconSet, forecast: &[DailyForecast]) -> Result<()> {
        for (index, rec) in self.rect.forecasts.iter().enumerate() {
            let daily = &forecast[index];
            let icon = get_icon_for_daily_forecast(icons, daily);
            let img_center_offset = Point::new((icons.WIDTH / 2) as i32, (icons.HEIGHT / 2) as i32);

            Image::new(icon, rec.bounding_box().center() - img_center_offset)
                .draw(&mut self.display.color_converted())?;

            let txt_offset = Point::new(0, (icons.HEIGHT / 2 + MARGIN) as i32);

            // Draw day of week
            let font = FontRenderer::new::<fonts::u8g2_font_profont22_tf>();
            let font_small = FontRenderer::new::<fonts::u8g2_font_profont17_tf>();
            let offset_dt = time::OffsetDateTime::from_unix_timestamp(daily.dt as i64)?;
            let format = time::format_description::parse("[weekday repr:short]")?;
            let day_formatted = offset_dt.format(&format)?;

            font.render_aligned(
                day_formatted.as_str(),
                rec.bounding_box().center() - txt_offset,
                VerticalPosition::Bottom,
                HorizontalAlignment::Center,
                FontColor::Transparent(BinaryColor::On),
                &mut self.display.color_converted(),
            ).unwrap();

            font_small.render_aligned(
                format_args!("{}°|{}°", daily.temp.min.round(), daily.temp.max.round()),
                rec.bounding_box().center() + txt_offset,
                VerticalPosition::Top,
                HorizontalAlignment::Center,
                FontColor::Transparent(BinaryColor::On),
                &mut self.display,
            ).unwrap();


        }
        Ok(())
    }


    fn debug_draw_rect(&mut self) -> Result<()> {
        let style = PrimitiveStyleBuilder::new()
            .stroke_color(BinaryColor::On)
            .stroke_width(1)
            .build();
        // self.rect.viewport.into_styled(style)
        //     .draw(&mut self.display.color_converted())?;
        // self.rect.current_weather.into_styled(style)
        //     .draw(&mut self.display.color_converted())?;
        // self.rect.weather_icon.into_styled(style)
        //     .draw(&mut self.display.color_converted())?;
        // self.rect.current_temp.into_styled(style)
        //     .draw(&mut self.display.color_converted())?;
        // self.rect.feels_like.into_styled(style)
        //     .draw(&mut self.display.color_converted())?;
        // self.rect.current_temp_unit.into_styled(style)
        //     .draw(&mut self.display.color_converted())?;
        // self.rect.date_location.into_styled(style)
        //     .draw(&mut self.display.color_converted())?;
        // self.rect.forecast.into_styled(style)
        //     .draw(&mut self.display.color_converted())?;
        //
        // for rec in self.rect.forecasts.iter() {
        //     rec.into_styled(style)
        //         .draw(&mut self.display.color_converted())?;
        // }

        self.rect.metrics.into_styled(style)
            .draw(&mut self.display.color_converted())?;
        self.rect.chart.into_styled(style)
            .draw(&mut self.display.color_converted())?;

        Ok(())
    }

}