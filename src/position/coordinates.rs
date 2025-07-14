use std::i8;
use std::ops::Add;

use strum_macros::EnumIter;
use crate::position::{board::Board};
use crate::position::color::Color;
use num_derive; 
use num_traits;

pub type SquareVec = Vec<Square>;
pub type CoordsVec = Vec<Coords>;

#[macro_export]
macro_rules! col {
    ($input:expr) => { Column::from($input) }       
}

#[macro_export]
macro_rules! row {
    ($input:expr) => { Row::from($input) }       
}

#[macro_export]
macro_rules! square {
    ($input:expr) => { Square::from($input) }       
}

#[macro_export]
macro_rules! coords {
    ($input:expr) => { Coords::from($input) } 
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

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Square { pub col: Column, pub row: Row,}
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coords { pub col: i8, pub row:i8}

pub fn from_checked_i8 <T: num_traits::FromPrimitive>(value: i8) -> T {
    num_traits::FromPrimitive::from_i8(value)
        .expect(&format!("failed to make {} out of {}", std::any::type_name::<T>(), value))
}

// i8 <> Row <> Column
impl From <i8> for Row {
    fn from(elt: i8) -> Self {
        from_checked_i8(elt)
    }
}

impl From <i8> for Column {
    fn from(elt: i8) -> Self {
        from_checked_i8(elt)
    }
}

impl From <Row> for i8 {
    fn from(r: Row) -> Self {
        r as i8
    }
}

impl From <Column> for i8 {
    fn from(c: Column) -> Self {
        c as i8
    }
}

// General Square and Coords methods
impl Square {

    pub fn get_color(&self) -> Color {
        let product: i8 = self.row as i8 * self.col as i8;
        match product % 2 == 0 {
            true  => Color::White,
            false => Color::Black
        }
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

// (i8, i8) <> Coords <> Square
impl From <Square> for Coords {
    fn from(square: Square) -> Self {
        Coords { col: square.col.into(), row: square.row.into() }
    }
}

impl From <Coords> for Square {
    fn from(coordinates: Coords) -> Self {
        assert!(coordinates.in_board(), "Tried to create Square from Coords that are out of board.");
        Square  {col: col!(coordinates.col), row: row!(coordinates.row) }
    }
}

impl From <(i8, i8)> for Coords {
    fn from(coord_couple: (i8, i8)) -> Self {
        Coords{ col: coord_couple.0, row: coord_couple.1 }
    }
}

impl From <(i8, i8)> for Square {
    fn from(coord_couple: (i8, i8)) -> Self {
        Square::from(Coords::from(coord_couple))
    }
}

impl From <Coords> for (i8, i8) {
    fn from(coords: Coords) -> Self {
        (coords.col as i8, coords.row as i8)
    }
}

impl From <Square> for (i8, i8) {
    fn from(square: Square) -> Self {
        Coords::from(square).into()
    }
}

impl From <(Column, Row)> for Square {
    fn from(cr: (Column, Row)) -> Self {
        Square { col: cr.0 , row: cr.1 }
    }
}

impl From <(Column, Row)> for Coords {
    fn from(cr: (Column, Row)) -> Self {
        Coords::from(Square::from(cr))
    }
}

// Additions for Coords

impl Add for Coords {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Coords{
            col: self.col + other.col,
            row: self.row + other.row }
    }
}

impl Add <(i8, i8)> for Coords {
    type Output = Self;
    fn add(self, added: (i8, i8) ) -> Self {
        Coords{
            col: self.col + added.0,
            row: self.row + added.1 }
    }
}

// Additions for Square

impl Add <Coords> for Square {
    type Output = Self;
    fn add(self, other: Coords) -> Self {
        let as_coords:Coords = self.into();
        Self::from(as_coords + other)
    }
}

impl Add <(i8, i8)> for Square {
    type Output = Self;
    fn add(self, added:(i8,i8)) -> Self {
        let as_coords = Coords::from(self) ;
        Square::from(as_coords + added)
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

pub trait SquareVecEquivalent {
    fn to_coords_vec(&self) -> SquareVec ;
}

impl SquareVecEquivalent for CoordsVec {
    fn to_coords_vec(&self) -> SquareVec {
        self.iter()
        .filter(|idx|idx.in_board())
        .map(|&coords|Square::from(coords))
        .collect()
    }
}