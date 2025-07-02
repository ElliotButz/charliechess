use crate::position::color::Color; 

pub struct Piece {
    color: Color,
    kind: PieceKind,
}

pub impl Piece {
    pub fn new(color: Color, kind: PieceKind) -> Piece {
        Piece {color, kind}
    }

}

pub enum PieceKind {
    Queen,
    Tower,
    Bishop,
    Knight,
    Pawn,
    King,
}

