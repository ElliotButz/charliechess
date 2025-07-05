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


#[derive(Clone, Copy)]
pub struct Piece {
    pub color: Color,
    pub kind: PieceKind,
}

impl Piece {
    pub fn new(color: Color, kind: PieceKind) -> Piece {
        Piece {color, kind}
    }

}

#[derive(Clone, Copy)]
pub enum PieceKind {
    Queen,
    Tower,
    Bishop,
    Knight,
    Pawn,
    King,
}

