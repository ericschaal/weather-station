use crate::display::display_driver::DisplayDriver;
use crate::display::{
    Display, DisplayConfig, BUFFER_SIZE, DISPLAY_AREA, DISPLAY_HEIGHT, DISPLAY_WIDTH,
};
use embedded_graphics::{geometry::*, pixelcolor::*, prelude::*, Pixel};
use esp_idf_sys::EspError;

pub struct PhysicalDisplay {
    driver: DisplayDriver,
    pixels: Vec<u8>,
    config: DisplayConfig,
}

impl Display for PhysicalDisplay {
    fn allow_out_of_bounds_drawing(&self) -> bool {
        self.config.allow_out_of_bounds_drawing
    }
    fn flush(&mut self) -> anyhow::Result<(), EspError> {
        self.driver.transmit_frame(&self.pixels)
    }
    fn refresh(&mut self) -> anyhow::Result<(), EspError> {
        self.driver.refresh()
    }
    fn flush_and_refresh(&mut self) -> anyhow::Result<(), EspError> {
        self.flush()?;
        self.refresh()
    }
    fn sleep(&mut self) -> anyhow::Result<(), EspError> {
        self.driver.sleep()
    }

    fn wake_up(&mut self) -> anyhow::Result<(), EspError> {
        self.driver.wake_up()
    }

    fn clear_screen(&mut self, clear_frame_buffer: bool) -> anyhow::Result<(), EspError> {
        if clear_frame_buffer {
            self.pixels = vec![BinaryColor::Off.into_storage(); BUFFER_SIZE];
        }
        self.driver.clear_screen()
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

impl PhysicalDisplay {
    pub fn new(
        driver: DisplayDriver,
        config: DisplayConfig,
    ) -> anyhow::Result<PhysicalDisplay, EspError> {
        Ok(PhysicalDisplay {
            driver,
            pixels: vec![BinaryColor::Off.into_storage(); BUFFER_SIZE],
            config,
        })
    }
}

impl OriginDimensions for PhysicalDisplay {
    fn size(&self) -> Size {
        DISPLAY_AREA.size
    }
}

impl DrawTarget for PhysicalDisplay {
    type Color = BinaryColor;
    type Error = core::convert::Infallible;
    fn draw_iter<I>(&mut self, pixels: I) -> anyhow::Result<(), Self::Error>
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
