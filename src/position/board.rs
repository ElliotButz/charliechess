use std::collections::HashMap;
use strum::IntoEnumIterator;

use crate::position::color::Color::{White, Black};
use crate::position::coordinates::{Coords, Row, Column};
use crate::position::pieces::{Piece, PieceKind::{Pawn, Knight, Bishop, Tower, Queen, King}};

pub struct Board {
    piece_by_coord: HashMap<Coords,Piece>,
}

impl Board {

    pub fn new() -> Board{
        let empty_piece_by_coords: HashMap<Coords,Piece>= HashMap::new();
        Board{piece_by_coord:empty_piece_by_coords}
    }

    pub fn initial_sate(&self) {
        for column in Column::iter(){ // Make pawns lines
            let row2 = Row::R2;
            let row7 = Row::R7;
            &self.add_piece_at_coords(Coords{col: column, row: row2}, Piece::new(White, Pawn));
            &self.add_piece_at_coords(Coords{col: column, row: row7}, Piece::new(Black, Pawn));
        };
    }

    pub fn piece_at_coords(&self, coords: Coords)-> Option<Piece> {
        let potential_piece = &self.piece_by_coord.get(&coords);
        match potential_piece {
            None => None,
            Some(&piece) => *potential_piece
        }

        return *piece.clone();
    }

    pub fn add_piece_at_coords(mut self,  coords: Coords, piece: Piece) {
        self.piece_by_coord.insert(coords, piece);
        ()
    }
}