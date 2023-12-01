use crate::display::{Display, BUFFER_SIZE, DISPLAY_AREA, DISPLAY_WIDTH};
use crate::http_client::post;
use anyhow::bail;
use embedded_graphics::{geometry::*, pixelcolor::*, prelude::*, Pixel};
use esp_idf_sys::EspError;

pub struct SimulatorDisplay {
    pixels: Vec<u8>,
}

impl SimulatorDisplay {
    pub fn new() -> anyhow::Result<Self, EspError> {
        return Ok(Self {
            pixels: vec![BinaryColor::Off.into_storage(); BUFFER_SIZE],
        });
    }
}

impl Display for SimulatorDisplay {
    fn allow_out_of_bounds_drawing(&self) -> bool {
        false
    }

    fn flush(&mut self) -> anyhow::Result<(), EspError> {
        post("http://192.168.2.14:8080/flush", &self.pixels).unwrap();
        Ok(())
    }

    fn refresh(&mut self) -> anyhow::Result<(), EspError> {
        post("http://192.168.2.14:8080/refresh", &self.pixels).unwrap();
        Ok(())
    }

    fn sleep(&mut self) -> anyhow::Result<(), EspError> {
        Ok(())
    }

    fn wake_up(&mut self) -> anyhow::Result<(), EspError> {
        Ok(())
    }

    fn clear_screen(&mut self, clear_frame_buffer: bool) -> anyhow::Result<(), EspError> {
        if clear_frame_buffer {
            self.pixels = vec![BinaryColor::Off.into_storage(); BUFFER_SIZE];
        }
        Ok(())
    }

    fn get_pixel(&self, p: Point) -> BinaryColor {
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

impl OriginDimensions for SimulatorDisplay {
    fn size(&self) -> Size {
        DISPLAY_AREA.size
    }
}

impl DrawTarget for SimulatorDisplay {
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
