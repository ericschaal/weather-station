use crate::chart::scales::linear::ScaleLinear;
use crate::config::CONFIG;
use crate::display::{Display, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use crate::icons::WeatherIconSet;
use crate::owm::icons::{get_icon_for_current_weather, get_icon_for_daily_forecast};
use crate::owm::model::{CurrentWeather, HourlyForecast};
use crate::owm::model::{DailyForecast, WeatherData};
use anyhow::Result;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::{geometry::*, image::*, prelude::*, primitives::*};
use itertools::Itertools;
use std::cmp::max;
use std::marker::PhantomData;
use u8g2_fonts::{fonts, types::*, FontRenderer};
const MARGIN: u32 = 8;
// Seems like icons have a bunch of padding on the horizontal axis
// This is a dirty attempt to gain some screen space
// Padding is actually closer to 32, but this is enough to make things fit
const IMG_ICON_PADDING: u32 = 16;

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

pub struct WeatherStationDraw<T>
where
    T: Display,
{
    rect: DisplayRect,
    large_icon_set: WeatherIconSet,
    small_icon_set: WeatherIconSet,
    _phantom: PhantomData<T>,
}

impl<T> Default for WeatherStationDraw<T>
where
    T: Display,
{
    fn default() -> Self {
        let viewport_size = Size::new(DISPLAY_WIDTH - MARGIN, DISPLAY_HEIGHT - MARGIN);
        let current_icon_size = Size::new(196 - IMG_ICON_PADDING, 196);
        let current_temp_size = Size::new(196, current_icon_size.height);

        let forecast_separator_size = Size::new(IMG_ICON_PADDING, current_icon_size.height);

        let current_weather_size = Size::new(
            current_temp_size.width + current_icon_size.width + forecast_separator_size.width,
            current_icon_size.height,
        );

        let temp_unit_size = Size::new(32, current_temp_size.height);
        let temp_feels_like_size = Size::new(current_temp_size.width, 32);

        let date_location_size = Size::new(viewport_size.width - current_weather_size.width, 64);
        let forecast_size = Size::new(
            viewport_size.width - current_weather_size.width,
            current_weather_size.height - date_location_size.height,
        );

        let metrics_size = Size::new(294, viewport_size.height - current_weather_size.height);
        let chart_size = Size::new(
            viewport_size.width - metrics_size.width,
            viewport_size.height - current_weather_size.height,
        );

        let viewport = Rectangle::new(Point::new(MARGIN as i32, MARGIN as i32), viewport_size);
        let current_weather = Rectangle::new(viewport.top_left, current_weather_size);
        let weather_icon = Rectangle::new(current_weather.top_left, current_icon_size);

        let current_temp = Rectangle::new(
            weather_icon.anchor_point(AnchorPoint::TopRight),
            current_temp_size,
        );
        let forecast_separator = Rectangle::new(
            current_temp.anchor_point(AnchorPoint::TopRight),
            forecast_separator_size,
        );

        let current_temp_unit = Rectangle::new(
            current_temp.anchor_point(AnchorPoint::TopRight),
            temp_unit_size,
        )
        .translate(Point::new(-(temp_unit_size.width as i32), 0));

        let feels_like = Rectangle::new(
            current_temp.anchor_point(AnchorPoint::BottomLeft),
            temp_feels_like_size,
        )
        .translate(Point::new(0, -(temp_feels_like_size.height as i32)));

        let date_location = Rectangle::new(
            forecast_separator.anchor_point(AnchorPoint::TopRight),
            date_location_size,
        );

        let forecast = Rectangle::new(
            date_location.anchor_point(AnchorPoint::BottomLeft),
            forecast_size,
        );
        let forecasts = [0; 5]
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let size = Size::new(forecast_size.width / 5, forecast_size.height);
                let offset_x = size.width * i as u32;
                Rectangle::new(
                    forecast.anchor_point(AnchorPoint::TopLeft) + Point::new(offset_x as i32, 0),
                    size,
                )
            })
            .collect::<Vec<_>>();

        let metrics = Rectangle::new(
            current_weather.anchor_point(AnchorPoint::BottomLeft),
            metrics_size,
        );
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

        let large_icon_set = WeatherIconSet::new().unwrap();
        let small_icon_set = WeatherIconSet::new_small().unwrap();

        WeatherStationDraw {
            rect,
            large_icon_set,
            small_icon_set,
            _phantom: PhantomData::default(),
        }
    }
}

impl<T> WeatherStationDraw<T>
where
    T: Display,
{
    pub fn draw_weather_report(&self, display: &mut T, weather: WeatherData) -> Result<()> {
        let app_config = CONFIG;
        let location_name = app_config.location_name;
        let current = weather.current.unwrap();
        let daily = weather.daily.unwrap();
        let hourly = weather.hourly.unwrap();
        let dt = current.dt;

        self.current_weather_icon(display, &current)?;
        self.current_temperature(display, &current)?;
        self.current_feels_like(display, &current)?;
        self.current_temp_unit(display)?;
        self.date_and_location(display, dt, location_name)?;
        self.daily_forecast(display, &daily)?;
        self.draw_chart(display, weather.timezone_offset, &hourly)?;

        self.debug_draw_rect(display)?;

        Ok(())
    }

    pub fn draw_ambient_data(&self, display: &mut T) -> Result<()> {
        Ok(())
    }

    fn current_weather_icon(&self, display: &mut T, current: &CurrentWeather) -> Result<()> {
        let icon = get_icon_for_current_weather(&self.large_icon_set, current);
        Image::new(
            icon,
            self.rect
                .current_weather
                .translate(Point::new(-(IMG_ICON_PADDING as i32) / 2, 0))
                .top_left,
        )
        .draw(&mut display.color_converted())?;
        Ok(())
    }

    fn current_temp_unit(&self, display: &mut T) -> Result<()> {
        let unit_style = PrimitiveStyleBuilder::new()
            .stroke_width(4)
            .stroke_color(BinaryColor::On)
            .build();

        let circle_diameter: u32 = 12;
        let circle_center = Point::new(circle_diameter as i32 / 2, circle_diameter as i32 / 2);
        let offset = Point::new(0, 46); // 46 is half the font size
        Circle::new(
            self.rect.current_temp_unit.center() - offset - circle_center,
            circle_diameter,
        )
        .into_styled(unit_style)
        .draw(&mut display.color_converted())?;

        Ok(())
    }

    fn current_feels_like(&self, display: &mut T, current: &CurrentWeather) -> Result<()> {
        let font = FontRenderer::new::<fonts::u8g2_font_profont22_tf>();

        font.render_aligned(
            format_args!("Feels Like {}°", current.feels_like.round() as i32),
            self.rect.feels_like.bounding_box().center(),
            VerticalPosition::Center,
            HorizontalAlignment::Center,
            FontColor::Transparent(BinaryColor::On),
            &mut display.color_converted(),
        )
        .unwrap();

        Ok(())
    }

    fn current_temperature(&self, display: &mut T, current: &CurrentWeather) -> Result<()> {
        let large_font = FontRenderer::new::<fonts::u8g2_font_logisoso92_tn>();

        large_font
            .render_aligned(
                format_args!("{}", current.temp.round() as i32),
                self.rect.current_temp.bounding_box().center(),
                VerticalPosition::Center,
                HorizontalAlignment::Center,
                FontColor::Transparent(BinaryColor::On),
                &mut display.color_converted(),
            )
            .unwrap();

        Ok(())
    }

    fn date_and_location(
        &self,
        display: &mut T,
        current_time: u64,
        location_name: &str,
    ) -> Result<()> {
        let large = FontRenderer::new::<fonts::u8g2_font_profont29_tf>();
        let font = FontRenderer::new::<fonts::u8g2_font_profont22_tf>();

        let offset_dt = time::OffsetDateTime::from_unix_timestamp(current_time as i64)?;
        let format = time::format_description::parse("[weekday], [day] [month repr:long] [year]")?;
        let formatted = offset_dt.format(&format)?;

        large
            .render_aligned(
                location_name,
                self.rect.date_location.anchor_point(AnchorPoint::TopRight),
                VerticalPosition::Top,
                HorizontalAlignment::Right,
                FontColor::Transparent(BinaryColor::On),
                display,
            )
            .unwrap();

        font.render_aligned(
            formatted.as_str(),
            self.rect.date_location.anchor_point(AnchorPoint::TopRight) + Point::new(0, 29),
            VerticalPosition::Top,
            HorizontalAlignment::Right,
            FontColor::Transparent(BinaryColor::On),
            display,
        )
        .unwrap();

        Ok(())
    }

    fn daily_forecast(&self, display: &mut T, forecast: &[DailyForecast]) -> Result<()> {
        let icons = &self.small_icon_set;

        for (index, rec) in self.rect.forecasts.iter().enumerate() {
            let daily = &forecast[index];
            let icon = get_icon_for_daily_forecast(icons, daily);
            let img_center_offset = Point::new((icons.WIDTH / 2) as i32, (icons.HEIGHT / 2) as i32);

            Image::new(icon, rec.bounding_box().center() - img_center_offset)
                .draw(&mut display.color_converted())?;

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
                display,
            )
            .unwrap();

            font_small
                .render_aligned(
                    format_args!("{}°|{}°", daily.temp.min.round(), daily.temp.max.round()),
                    rec.bounding_box().center() + txt_offset,
                    VerticalPosition::Top,
                    HorizontalAlignment::Center,
                    FontColor::Transparent(BinaryColor::On),
                    display,
                )
                .unwrap();
        }
        Ok(())
    }

    fn draw_chart(
        &self,
        display: &mut T,
        timezone_offset: i64,
        forecast: &Vec<HourlyForecast>,
    ) -> Result<()> {
        let app_config = CONFIG;

        let temp = forecast
            .iter()
            .map(|hourly| Point {
                x: (hourly.dt) as i32,
                y: hourly.temp.round() as i32,
            })
            .take(app_config.hours_to_draw)
            .collect::<Vec<_>>();

        // From 0 to 100%
        let precipitation = forecast
            .iter()
            .map(|hourly| Point {
                x: (hourly.dt) as i32,
                y: hourly.pop as i32 * 100,
            })
            .take(app_config.hours_to_draw)
            .collect::<Vec<_>>();

        let (x_min, x_max) = temp.iter().map(|p| p.x).minmax().into_option().unwrap();
        let (temp_min, temp_max) = temp.iter().map(|p| p.y).minmax().into_option().unwrap();

        // Try to have a 20° range
        let temp_range = {
            let new_min = temp_min;
            let new_max = max(temp_max, new_min + 20);

            new_min..=new_max
        };

        let precip_range = { 0.0..=100.0 };

        // Layout rectangles
        let margin: u32 = 4;
        let axis_rec_dim = 16;
        let curve_size = Size::new(
            self.rect.chart.size.width - 2 * margin - 2 * axis_rec_dim,
            self.rect.chart.size.height - 2 * margin - axis_rec_dim,
        );

        let left_axis_rec = Rectangle::new(
            self.rect.chart.top_left + Point::new(margin as i32, margin as i32),
            Size::new(axis_rec_dim, curve_size.height),
        );

        let curve_rec = Rectangle::new(
            left_axis_rec.anchor_point(AnchorPoint::TopRight),
            Size::new(curve_size.width, curve_size.height),
        );

        let bottom_axis_rec = Rectangle::new(
            curve_rec.anchor_point(AnchorPoint::BottomLeft),
            Size::new(curve_size.width, axis_rec_dim),
        );

        let right_axis_rec = Rectangle::new(
            curve_rec.anchor_point(AnchorPoint::TopRight),
            Size::new(axis_rec_dim, curve_size.height),
        );

        let style = PrimitiveStyleBuilder::new()
            .stroke_color(BinaryColor::On)
            .stroke_width(1)
            .build();

        curve_rec
            .into_styled(style)
            .draw(&mut display.color_converted())?;
        left_axis_rec
            .into_styled(style)
            .draw(&mut display.color_converted())?;
        right_axis_rec
            .into_styled(style)
            .draw(&mut display.color_converted())?;
        curve_rec
            .into_styled(style)
            .draw(&mut display.color_converted())?;
        bottom_axis_rec
            .into_styled(style)
            .draw(&mut display.color_converted())?;

        for row_index in 0..5 {
            let spacing = curve_rec.size.height / 5;
            let v_offset = Point::new(0, (spacing * row_index) as i32);
            Line::new(
                curve_rec.top_left + v_offset,
                curve_rec.anchor_point(AnchorPoint::TopRight) + v_offset,
            )
            .into_styled(style)
            .draw(&mut display.color_converted())?;
        }

        let x_scale = ScaleLinear::new()
            .set_domain(vec![x_min as f32, x_max as f32])
            .set_range(vec![
                curve_rec.anchor_point(AnchorPoint::TopLeft).x as f32,
                curve_rec.anchor_point(AnchorPoint::TopRight).x as f32,
            ]);

        let temp_scale = ScaleLinear::new()
            .set_domain(vec![*temp_range.end() as f32, *temp_range.start() as f32])
            .set_range(vec![
                curve_rec.anchor_point(AnchorPoint::TopRight).x as f32,
                curve_rec.anchor_point(AnchorPoint::BottomRight).x as f32,
            ]);

        let precip_scale = ScaleLinear::new()
            .set_domain(vec![
                *precip_range.end() as f32,
                *precip_range.start() as f32,
            ])
            .set_range(vec![
                curve_rec.anchor_point(AnchorPoint::TopRight).x as f32,
                curve_rec.anchor_point(AnchorPoint::BottomRight).x as f32,
            ]);

        Ok(())
    }

    fn debug_draw_rect(&self, display: &mut T) -> Result<()> {
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

        self.rect
            .metrics
            .into_styled(style)
            .draw(&mut display.color_converted())?;
        self.rect
            .chart
            .into_styled(style)
            .draw(&mut display.color_converted())?;

        Ok(())
    }
}
