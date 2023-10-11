use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::Drawable;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::primitives::Rectangle;


pub struct Chart<'a, Element: Drawable<Color=BinaryColor, Output=()>> {
    viewport: Rectangle,
    elements: Vec<&'a Element>
}


impl<'a, Element> Chart<'a, Element>
    where Element: Drawable<Color=BinaryColor, Output=()> {
    pub fn new(viewport: Rectangle) -> Self {
        Self {
            viewport,
            elements: Vec::new(),
        }
    }

    pub fn with_element(mut self, element: &'a Element) -> Self {
        self.elements.push(element);
        self
    }

}

impl<'a, Element> Drawable for Chart<'a, Element>
    where Element: Drawable<Color=BinaryColor, Output=()>{
    type Color = BinaryColor;
    type Output = ();
    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error> where D: DrawTarget<Color=Self::Color> {
        for element in self.elements.iter() {
            element.draw(target)?;
        }
        Ok(())
    }
}