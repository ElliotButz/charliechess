use std::collections::HashMap;
use strum::IntoEnumIterator;
use colored::{ColoredString, Colorize};


use crate::{coords, piece};
use crate::position::color::{Color,Color::{White, Black}};
use crate::position::coordinates::{Coords, Row, Row::*, Column, Column::*};
use crate::position::pieces::{Piece, PieceKind, PieceKind::{Pawn, Knight, Bishop, Tower, Queen, King}};

pub type BoardMap = HashMap<Coords,Piece>;
pub struct Board {
    map: BoardMap,
    // TODO: Add history features : black_king_has_move, black_Hrook_has_moved...
}

impl Board { // Initiators and init helpers

    pub fn new() -> Board { // Initiator
        Board { map: BoardMap::with_capacity(64) }
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
        piece_at_coords
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
    }
}


impl Board {
    pub fn terminal_display(&self) {

        let mut board_str = String::from(".  .  .  .  .  .  .  .  .\n");
        for row in Row::iter(){
            for col in Column::iter(){

                let case_color:Color = coords!(col, row).get_color();
                let piece_char = match &self.piece_at_coords(coords!(col, row)) {
                    Some(piece_at_pos) => piece_at_pos.as_char(),
                    None => ' '
                };
                let piece_str = String::from(piece_char);
                let piece_on_case_str: ColoredString = match case_color {
                    White => piece_str.on_color("white"),
                    Black => piece_str.on_color("grey")
                };
                board_str.push_str(".");
                board_str.push_str(&piece_on_case_str);
                board_str.push_str(" ");
            }
            board_str.push_str(".\n");
        }
        println!("{}",board_str);
    }
}