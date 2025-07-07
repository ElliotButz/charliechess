use std::u8;

use strum_macros::EnumIter;
use crate::position::color::Color;
use num_derive; 
use num_traits;

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

#[derive(EnumIter, PartialEq, Eq, Hash, Clone, Copy, num_derive::FromPrimitive, num_derive::ToPrimitive)]
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

#[derive(EnumIter, PartialEq, Hash, Eq,  Clone, Copy,  num_derive::FromPrimitive, num_derive::ToPrimitive)]
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

fn idx<T: num_traits::ToPrimitive>(elt: T) -> u8 {
    elt.to_u8().expect("Enum value must fit in u8")
}

fn from_idx <T: num_traits::FromPrimitive>(idx:u8) -> T {
    num_traits::FromPrimitive::from_u8(idx).expect("Failed to create enum value out of an u8.")
}


impl Coords {
    pub fn get_color(&self) -> Color {
        let product = idx(self.row)*idx(self.col);
        match product % 2 == 0 {
            true  => Color::White,
            false => Color::Black
        }
    }

    pub fn to_colrow_idx(&self) -> (u8,u8) {
        (idx(self.col),idx(self.row))
    }

    pub fn from_cartesian(c: u8, r: u8) -> Self {
        Coords {
            col:from_idx(c), row: from_idx(r),
        }
    }
}