use std::fmt;
use crate::position::coordinates::types_and_structs::{Row, Column, Square};
use crate::position::pieces::{Piece,PieceKind};
use crate::position::color::Color;
use crate::square;

#[derive(Clone, Copy)]
pub enum CoupKind { Normal, Castle }


#[derive(Clone, Copy)]
pub struct Coup {
    pub start: Square,
    pub end  : Square,
    pub piece: Piece,
    pub taken: Option<Piece>,
    pub kind : CoupKind
    // pub checks: bool
}

impl Coup {

    pub fn is_pawn_double_step(&self) -> bool {
        self.piece.kind == PieceKind::Pawn &&
        (self.start.row as i8 - self.end.row as i8).abs() == 2
    }

    pub fn coup_zero() -> Self {
        Self {
            start: square!((Column::C, Row::R3)),
            end:   square!((Column::B, Row::R1)),
            piece: Piece {color: Color::White, kind: PieceKind::Knight},
            taken: None,
            kind : CoupKind::Normal
            // checks: false
        }
    }
}

impl fmt::Display for Coup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}-{}", self.start, self.end)
    }
}



/* impl Turn {
    pub fn to_str(&self) -> String {
        let mut description = String::new("");

    }
} */