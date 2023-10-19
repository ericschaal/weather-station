pub mod line;

use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::Drawable;

pub trait ChartElement {
    type DrawableType: Drawable<Color = BinaryColor, Output = ()>;
    fn to_drawable(&self) -> Self::DrawableType;
}
