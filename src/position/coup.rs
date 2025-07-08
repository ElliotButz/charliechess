use crate::position::coordinates::Coords;
use crate::position::pieces::Piece;

#[derive(Clone, Copy)]
pub struct Coup {
    pub start:  Coords,
    pub end:    Coords,
    pub piece:  Piece,
    pub taken:  Option<Piece>,
    pub checks: bool
}

#[derive(Clone, Copy)]
pub struct Turn {
    pub white_coup: Coup,
    pub black_coup: Option<Coup> // None if game ends after white's move.
} 

/* impl Turn {
    pub fn to_str(&self) -> String {
        let mut description = String::new("");

    }
} */