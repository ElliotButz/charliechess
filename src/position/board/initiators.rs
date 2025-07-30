use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use fnv::FnvHasher;

use strum::IntoEnumIterator;
use crate::position::board::types_and_structs::BoardMap;
use crate::position::coup::Coup;
use crate::{square, piece};
use crate::position::coordinates::types_and_structs::{Column, Column::*, Row::*, SquareVec};
use crate::position::color::Color::{White, Black};
use crate::position::pieces::{Piece, PieceKind, PieceKind::{Pawn, Knight, Bishop, Tower, Queen, King}};
use crate::position::board::types_and_structs::Board;

type FnvHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FnvHasher>>;

impl Board { // Initiators and init helpers

    pub fn new() -> Board { // Initiator
        let new_board = Board {
            map: BoardMap::with_capacity(64),
            player_to_play: White,
            last_move: Coup::coup_zero(),
            black_watchers_and_watched: HashMap::new(),
            white_watchers_and_watched: HashMap::new(),
            possible_moves: Vec::new(),
            black_king_is_checked: false,
            white_king_is_checked: false,
            black_can_h_castle: false,
            black_can_a_castle: false,
            white_can_h_castle: false,
            white_can_a_castle: false,
            black_king_has_moved: false,
            white_king_has_moved: false,
            h_black_tower_has_moved: false,
            a_black_tower_has_moved: false,
            h_white_tower_has_moved: false,
            a_white_tower_has_moved: false,
/*             squares_with_pined_pieces  : SquareVec::with_capacity(8),
            squares_with_pining_pieces : SquareVec::with_capacity(8), */
        };
        new_board
    }

    pub fn from_boardmap(piece_by_coords:BoardMap) -> Board{ // Initiator
        let mut board = Board::new();
        board.map = piece_by_coords;
        board.update_info(2);
        board
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

            piece_at_coords.insert(square!((col, R8)), piece!(Black, major_piece_kind)); // Black major pieces
            piece_at_coords.insert(square!((col, R7)), piece!(Black, Pawn)); // Black pawns
            piece_at_coords.insert(square!((col, R2)), piece!(White, Pawn)); // White pawns
            piece_at_coords.insert(square!((col, R1)), piece!(White, major_piece_kind)); // White major pieces

        };
        piece_at_coords
    }
}