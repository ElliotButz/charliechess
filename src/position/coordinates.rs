use std::i8;
use std::ops::Add;

use strum_macros::EnumIter;
use crate::position::color::Color;
use num_derive; 
use num_traits;

pub type CoordsVec = Vec<Coords>; 

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

fn idx<T: num_traits::ToPrimitive>(elt: T) -> i8 {
    elt.to_i8().expect("Enum value must fit in i8")
}

fn from_idx <T: num_traits::FromPrimitive>(idx:i8) -> T {
    num_traits::FromPrimitive::from_i8(idx).expect("Failed to create enum value out of an i8.")
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct IdxCoordinates {col: i8, row:i8}

impl Coords {
    pub fn get_color(&self) -> Color {
        let product = idx(self.row)*idx(self.col);
        match product % 2 == 0 {
            true  => Color::White,
            false => Color::Black
        }
    }

    pub fn to_colrow_idx(&self) -> IdxCoordinates {
         IdxCoordinates{col: idx(self.col),row: idx(self.row)}
    }

    pub fn to_colidx_rowidx(&self) -> (i8, i8) {
        let coordsidx = Self::to_colrow_idx(&self);
        (coordsidx.col, coordsidx.row)
    }

    pub fn from_colrow_idx(idx_coordinates:  &IdxCoordinates) -> Self {
        assert!(idx_coordinates.is_oob(), "Tried to create Coords from IdxCoordinates that are out of board.");
        Coords {
            col:from_idx(idx_coordinates.col), row: from_idx(idx_coordinates.row),
        }
    }
    pub fn from_colidx_rowidx(c: i8, r: i8) -> Self {
        let coords = IdxCoordinates{ col: c, row: r };
        Self::from_colrow_idx(&coords)
    }
}

impl Add for IdxCoordinates {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let col_sum = self.col + other.col;
        let row_sum = self.row + other.row;
        IdxCoordinates{ col: col_sum , row:row_sum }
    }
}

impl IdxCoordinates {
    pub fn is_oob(&self) -> bool { // oob : out of board
        !(1..8).contains(&self.col) && !(1..8).contains(&self.row) 
    }
}

impl Add for Coords {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::from_colrow_idx(&(self.to_colrow_idx() + other.to_colrow_idx()))
    }
}

