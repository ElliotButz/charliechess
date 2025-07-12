use crate::position::coordinates::{Column, Row, Coords};
use crate::position::pieces::{Piece,PieceKind};
use crate::position::color::Color;
use crate::coords;

#[derive(Clone, Copy)]
pub struct Coup {
    pub start:  Coords,
    pub end:    Coords,
    pub piece:  Piece,
    pub taken:  Option<Piece>,
    pub checks: bool
}

impl Coup {

    pub fn is_pawn_double_step(&self) -> bool {
        self.piece.kind == PieceKind::Pawn &&
        (self.start.to_colrow_idx().row-self.end.to_colrow_idx().row).abs() == 2
    }

    pub fn coup_zero() -> Self {
        Self {
            start:  coords!(Column::C, Row::R3),
            end:    coords!(Column::B, Row::R1),
            piece:  Piece {color: Color::White, kind: PieceKind::Knight},
            taken:  None,
            checks: false
        }
    }
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