use std::ops::Add;

use crate::coords;
use crate::position::{color::Color, coordinates::types_and_structs::{Coords, Square}};

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

// Operation for Coords

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

impl std::ops::Mul<i8> for Coords {
    type Output = Self;
    fn mul(self, m: i8) -> Self {
        let (colidx, rowidx): (i8, i8) = self.into();
        coords!((colidx*m, rowidx*m))
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