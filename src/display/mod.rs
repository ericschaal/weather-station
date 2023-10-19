pub mod command;
pub mod display_driver;
pub mod traits;

use crate::display::display_driver::DisplayDriver;
use anyhow::Result;
use embedded_graphics::{geometry::*, pixelcolor::*, prelude::*, primitives::Rectangle, Pixel};
use esp_idf_sys::EspError;

pub const DISPLAY_HEIGHT: u32 = 480;
pub const DISPLAY_WIDTH: u32 = 800;
const DISPLAY_AREA: Rectangle =
    Rectangle::new(Point::zero(), Size::new(DISPLAY_WIDTH, DISPLAY_HEIGHT));
const DEFAULT_BACKGROUND_COLOR: BinaryColor = BinaryColor::Off;
const BITS_PER_PIXEL: u32 = 1;
pub const BUFFER_SIZE: usize = (DISPLAY_WIDTH * DISPLAY_HEIGHT / (8 * BITS_PER_PIXEL)) as usize;

pub struct DisplayConfig {
    pub allow_out_of_bounds_drawing: bool,
}

pub struct Display {
    driver: DisplayDriver,
    pixels: Vec<u8>,
    config: DisplayConfig,
}

impl Display {
    pub fn new(driver: DisplayDriver, config: DisplayConfig) -> Result<Display, EspError> {
        Ok(Display {
            driver,
            pixels: vec![BinaryColor::Off.into_storage(); BUFFER_SIZE],
            config,
        })
    }
    pub fn flush(&mut self) -> Result<(), EspError> {
        self.driver.transmit_frame(&self.pixels)
    }
    pub fn refresh(&mut self) -> Result<(), EspError> {
        self.driver.refresh()
    }
    pub fn flush_and_refresh(&mut self) -> Result<(), EspError> {
        self.flush()?;
        self.refresh()
    }

    pub fn sleep(&mut self) -> Result<(), EspError> {
        self.driver.sleep()
    }
    pub fn wake_up(&mut self) -> Result<(), EspError> {
        self.driver.wake_up()
    }

    pub fn clear_screen(&mut self, clear_frame_buffer: bool) -> Result<(), EspError> {
        if clear_frame_buffer {
            self.pixels = vec![BinaryColor::Off.into_storage(); BUFFER_SIZE];
        }
        self.driver.clear_screen()
    }

    pub fn set_pixel(&mut self, point: Point, color: BinaryColor) {
        assert!(
            point.x >= 0
                && point.y >= 0
                && point.x < DISPLAY_WIDTH as i32
                && point.y < DISPLAY_HEIGHT as i32,
            "point must be inside display bounding box: {:?}",
            point
        );
        self.set_pixel_unchecked(point, color)
    }

    pub fn set_pixels(&mut self, points: impl IntoIterator<Item = Point>, color: BinaryColor) {
        for point in points {
            self.set_pixel(point, color);
        }
    }

    pub fn draw_pixel(&mut self, point: Point, color: BinaryColor) {
        if !DISPLAY_AREA.contains(point) {
            if !self.config.allow_out_of_bounds_drawing {
                panic!(
                    "tried to draw pixel outside the display area (x: {}, y: {})",
                    point.x, point.y
                );
            } else {
                return;
            }
        }

        self.set_pixel_unchecked(point, color);
    }

    pub fn get_pixel(&self, p: Point) -> BinaryColor {
        let index = (p.x as u32 + p.y as u32 * DISPLAY_WIDTH) as usize;
        let byte_index = index / 8;
        let bit_index = index % 8;

        if self.pixels[byte_index] & (0x80 >> bit_index) != 0 {
            BinaryColor::On
        } else {
            BinaryColor::Off
        }
    }

    fn set_pixel_unchecked(&mut self, point: Point, color: BinaryColor) {
        let index = (point.x as u32 + point.y as u32 * DISPLAY_WIDTH) as usize;
        let byte_index = index / 8;
        let bit_index = index % 8;

        match color {
            BinaryColor::Off => {
                // Clear the bit for "off" (white) pixel
                self.pixels[byte_index] &= !(0x80 >> bit_index);
            }
            BinaryColor::On => {
                // Set the bit for "on" (black) pixel
                self.pixels[byte_index] |= 0x80 >> bit_index;
            }
        }
    }
}

impl OriginDimensions for Display {
    fn size(&self) -> Size {
        DISPLAY_AREA.size
    }
}

impl DrawTarget for Display {
    type Color = BinaryColor;
    type Error = core::convert::Infallible;
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for pixel in pixels.into_iter() {
            let Pixel(point, color) = pixel;

            self.draw_pixel(point, color);
        }

        Ok(())
    }
}
