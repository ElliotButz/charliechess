use crate::position::color::Color; 

#[derive(Clone, Copy)]
pub struct Piece {
    color: Color,
    kind: PieceKind,
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

