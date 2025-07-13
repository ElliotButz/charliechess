use std::i8;
use std::ops::Add;

use strum_macros::EnumIter;
use crate::position::board::Board;
use crate::position::color::Color;
use num_derive; 
use num_traits;

pub type SquareVec = Vec<Square>;
pub type CoordsVec = Vec<Coords>;


#[macro_export]
macro_rules! square {
    ($col:expr, $row:expr) => {
        Square {
            col: $col,
            row: $row,
        }
    };
}
#[macro_export]
macro_rules! idxcoords {
    ($col:expr, $row:expr) => {
        Coords {
            col: $col,
            row: $row,
        }
    };
}


#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Square {
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

pub fn coord<T: num_traits::ToPrimitive>(elt: T) -> i8 {
    elt.to_i8().expect("Enum value must fit in i8")
}

pub fn from_coord <T: num_traits::FromPrimitive>(idx:i8) -> T {
    num_traits::FromPrimitive::from_i8(idx).expect("Failed to create enum value out of an i8.")
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coords { pub col: i8, pub row:i8}


impl Square {
    pub fn get_color(&self) -> Color {
        let product = coord(self.row)*coord(self.col);
        match product % 2 == 0 {
            true  => Color::White,
            false => Color::Black
        }
    }

    pub fn to_coords(&self) -> Coords {
         Coords{col: coord(self.col),row: coord(self.row)}
    }

    pub fn to_coord_couple(&self) -> (i8, i8) {
        let coordsidx = Self::to_coords(&self);
        (coordsidx.col, coordsidx.row)
    }

    pub fn from_coords(idx_coordinates:  Coords) -> Self {
        assert!(idx_coordinates.in_board(), "Tried to create Coords from IdxCoords that are out of board.");
        Square {
            col:from_coord(idx_coordinates.col), row: from_coord(idx_coordinates.row),
        }
    }
    pub fn from_coord_couple(c: i8, r: i8) -> Self {
        let coords = Coords{ col: c, row: r };
        Self::from_coords(coords)
    }
}

impl Add for Coords {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Coords{
            col: self.col + other.col,
            row: self.row + other.row }
    }
}

impl Coords {
    pub fn in_board(&self) -> bool { 
        (1..=8).contains(&self.col) && (1..=8).contains(&self.row) 
    }

    pub fn not_in_board(&self) -> bool {
        !Self::in_board(&self)
    }
}

impl Add for Square {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::from_coords((self.to_coords() + other.to_coords()))
    }
}

pub trait ExcludeOutOfBoard {
    fn exclude_out_of_board(&mut self) ;
}

impl ExcludeOutOfBoard for CoordsVec{
    fn exclude_out_of_board(&mut self) {
        self.retain(|&coords| coords.in_board())
    }
}

pub trait OpenToColor {
    fn open_to_color(&mut self, board: &Board, color: Color);
}
impl OpenToColor for SquareVec {
    fn open_to_color(&mut self, board: &Board, color: Color) {
        self.retain(|&square| board.square_is_free_for_piece_of_color(square, color))
    }
}

pub trait CoordsVecEquivalent {
    fn to_coords_vec(&self) -> SquareVec ;
}

impl CoordsVecEquivalent for CoordsVec {
    fn to_coords_vec(&self) -> SquareVec {
        self.iter()
        .filter(|idx|idx.in_board())
        .map(|&coords|Square::from_coords(coords))
        .collect()
    }
}