use std::collections::HashMap;
use strum::IntoEnumIterator;

use crate::{coords, piece};
use crate::position::color::Color::{White, Black};
use crate::position::coordinates::{Coords, Row::*, Column, Column::*};
use crate::position::pieces::{Piece, PieceKind, PieceKind::{Pawn, Knight, Bishop, Tower, Queen, King}};

pub type BoardMap = HashMap<Coords,Piece>;
pub struct Board {
    map: BoardMap,
}

impl Board { // Initiators and init helpers

    pub fn new() -> Board { // Initiator
        return Board { map: BoardMap::with_capacity(64) }
    }

    pub fn from_boardmap(piece_by_coords:BoardMap) -> Board{ // Initiator
        Board{map:piece_by_coords}
    }

    pub fn at_start_state() -> Board { // Initiator
        Board::from_boardmap(Board::make_start_state())
    }

    pub fn make_start_state() -> BoardMap { // init Helper

        let mut piece_at_coords= BoardMap::with_capacity(64);
        for col in Column::iter(){
            
            let major_piece_kind:PieceKind = match col {
                A | H => Tower,
                B | G => Knight,
                C | F => Bishop,
                D => Queen,
                E => King,
            };

            piece_at_coords.insert(coords!(col, R8), piece!(Black, major_piece_kind)); // Black major pieces
            piece_at_coords.insert(coords!(col, R7), piece!(Black, Pawn)); // Black pawns
            piece_at_coords.insert(coords!(col, R2), piece!(White, Pawn)); // White pawns
            piece_at_coords.insert(coords!(col, R1), piece!(White, major_piece_kind)); // White major pieces

        };
        return piece_at_coords
    }
}

impl Board { // Requesters
    pub fn piece_at_coords(&self, coords: Coords)-> Option<Piece> {
        self.map.get(&coords).copied()
    }
}

impl Board { // Editors
    pub fn add_piece_at_coords(&mut self,  coords: Coords, piece: Piece) {
        self.map.insert(coords, piece);
        ()
    }
}