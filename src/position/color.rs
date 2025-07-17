#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {White=1,Black=-1}

impl Color {
    pub fn as_direction(&self) -> i8 {
        match self {
            Color::White => 1i8,
            Color::Black => -1i8
        }}

    pub fn the_other(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White
        }
    } 
}