use strum_macros::EnumIter;
use crate::position::color::Color;

#[macro_export]
macro_rules! coords {
    ($col:ident, $row:ident) => {
        Coords {
            col: $col,
            row: $row,
        }
    };
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coords {
    pub col: Column,
    pub row: Row,
}

#[derive(EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Row {
    R1=1,
    R2=2,
    R3=3,
    R4=4,
    R5=5,
    R6=6,
    R7=7,
    R8=8,
}

#[derive(EnumIter, PartialEq, Hash, Eq,  Clone, Copy)]
pub enum Column{
    A=1,
    B=2,
    C=3,
    D=4,
    E=5,
    F=6,
    G=7,
    H=8,
}

pub fn row_idx(row:&Row) -> u8 {
    *row as u8
}

pub fn col_idx(col:&Column) -> u8 {
    *col as u8
}

impl Coords {
    pub fn get_color(&self) -> Color {
        let product = row_idx(&self.row)*col_idx(&self.col);
        match product % 2 == 0 {
            true  => Color::White,
            false => Color::Black
        }
    }
}