pub mod command;
pub mod display_driver;
pub mod physical;
#[cfg(feature = "display-simulator")]
pub mod simulator;
pub mod traits;

use anyhow::Result;
use embedded_graphics::prelude::DrawTarget;
use embedded_graphics::{geometry::*, pixelcolor::*, primitives::Rectangle};
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

pub trait Display:
    DrawTarget<Color = BinaryColor, Error = core::convert::Infallible> + OriginDimensions
{
    fn allow_out_of_bounds_drawing(&self) -> bool;
    fn flush(&mut self) -> Result<(), EspError>;
    fn refresh(&mut self) -> Result<(), EspError>;
    fn flush_and_refresh(&mut self) -> anyhow::Result<(), EspError> {
        self.flush()?;
        self.refresh()
    }
    fn sleep(&mut self) -> Result<(), EspError>;
    fn wake_up(&mut self) -> Result<(), EspError>;
    fn clear_screen(&mut self, clear_frame_buffer: bool) -> Result<(), EspError>;
    fn get_pixel(&self, p: Point) -> BinaryColor;
    fn set_pixel_unchecked(&mut self, point: Point, color: BinaryColor);

    fn set_pixel(&mut self, point: Point, color: BinaryColor) {
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

    fn set_pixels(&mut self, points: impl IntoIterator<Item = Point>, color: BinaryColor) {
        for point in points {
            self.set_pixel(point, color);
        }
    }

    fn draw_pixel(&mut self, point: Point, color: BinaryColor) {
        if !DISPLAY_AREA.contains(point) {
            if !self.allow_out_of_bounds_drawing() {
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
}
