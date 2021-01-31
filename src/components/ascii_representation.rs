use amethyst::ecs::{Component, DefaultVecStorage};

pub type Color = i16;

#[derive(Clone, Copy)]
pub struct AsciiRepresentation (char, Color);

impl AsciiRepresentation {
    pub fn new(c: char, col: Color) -> Self{
        AsciiRepresentation(c, col)
    }

    pub fn get_char(&self) -> char{
        self.0
    }

    pub fn get_color(&self) -> Color{
        self.1
    }
}

impl Component for AsciiRepresentation{
    type Storage = DefaultVecStorage<Self>;
}

impl Default for AsciiRepresentation{
    fn default() -> Self {
        AsciiRepresentation(' ', 0)
    }
}
