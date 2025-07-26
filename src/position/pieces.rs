use std::fmt;

use strum_macros::Display;
use crate::position::color::Color; 

#[macro_export]
macro_rules! piece {
    ($color:expr, $kind:expr) => {
        Piece {
            color: $color,
            kind: $kind,
        }
    };
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    pub color: Color,
    pub kind: PieceKind,
}

impl Piece {
    pub fn new(color: Color, kind: PieceKind) -> Piece {
        Piece {color, kind}
    }

    pub fn as_char(&self) -> char {
        let (kind, color) = (self.kind, self.color);
        match (kind, color) {
            (PieceKind::Queen,  Color::Black) => '\u{265B}',
            (PieceKind::King,   Color::Black) => '\u{265A}',
            (PieceKind::Bishop, Color::Black) => '\u{265D}',
            (PieceKind::Knight, Color::Black) => '\u{265E}',
            (PieceKind::Tower,  Color::Black) => '\u{265C}',
            (PieceKind::Pawn,   Color::Black) => '\u{265F}',
            (PieceKind::Queen,  Color::White) => '\u{2655}',
            (PieceKind::King,   Color::White) => '\u{2654}',
            (PieceKind::Bishop, Color::White) => '\u{2657}',
            (PieceKind::Knight, Color::White) => '\u{2658}',
            (PieceKind::Tower,  Color::White) => '\u{2656}',
            (PieceKind::Pawn,   Color::White) => '\u{2659}',
        }
        
    }
}

#[derive(Clone, Copy, Display, Eq, PartialEq, Debug, Hash)]
pub enum PieceKind {
    Queen,
    Tower,
    Bishop,
    Knight,
    Pawn,
    King,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.color, self.kind)
    }
}
