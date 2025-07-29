use std::collections::HashMap;

use ordered_hash_map::OrderedHashMap;

use crate::position::coordinates::types_and_structs::{Coords, CoordsVec, Row, Square, SquareVec};
use crate::position::coordinates::converters::{to_square_vec};
use crate::position::color::Color;
use crate::position::coup::{Coup, CoupKind};
use crate::position::pieces::{Piece, PieceKind::{Bishop, King, Queen, Tower, Pawn, Knight}};
use crate::position::board::types_and_structs::Board;
use crate::position::basic_piece_moves::*;

impl Board {
    fn update_castle_rights(& mut self) {
       
        self.white_can_a_castle = self.can_castle(
            Color::White,
            self.a_white_tower_has_moved,
            self.white_king_has_moved,
            1,
            4
        );
        self.white_can_h_castle = self.can_castle(
            Color::White,
            self.h_white_tower_has_moved,
            self.white_king_has_moved,
            5,
            7
        );
        self.black_can_a_castle = self.can_castle(
            Color::Black,
            self.a_black_tower_has_moved,
            self.black_king_has_moved,
            1,
            4
        );
        self.black_can_h_castle = self.can_castle(
            Color::Black,
            self.h_black_tower_has_moved,
            self.black_king_has_moved,
            5,
            7
        );
    }

    fn update_king_safety(&mut self) {

        for player_color in [Color::White, Color::Black] {
            println!("KINGSAF\n{}", {&self});
            let king_is_checked = self.is_checked(player_color);
            match player_color {
                Color::Black => self.black_king_is_checked = king_is_checked,
                Color::White => self.white_king_is_checked = king_is_checked
            }
        }
    }
    
    /// Indicates squares watched (values) by each square with a piece of the player with trait (key).
    pub fn update_watchers_sites_and_watched_squares(&mut self) {
        self.white_watchers_and_watched = HashMap::new();
        self.black_watchers_and_watched = HashMap::new();
        for (&square, &piece) in &self.map {
            let watched =
                match piece.kind{
                    Bishop => bishop_moves::watched_squares(self, square),
                    Tower  =>  tower_moves::watched_squares(self, square),
                    Queen  =>  queen_moves::watched_squares(self, square),
                    Pawn   =>   pawn_moves::watched_squares(square, piece.color),
                    Knight => knight_moves::watched_squares(square),
                    King   =>   king_moves::watched_squares(square),
                };
            match piece.color {
                Color::Black => { self.black_watchers_and_watched.insert(square, watched); }
                Color::White => { self.white_watchers_and_watched.insert(square, watched); }
            }

        };
    }


    pub fn update_possible_moves(&mut self) {
        // Returns all possible moves (aka coups) for player to move.
        // The board infos should be up to date (use self.update_info()).

        let mut moves: Vec<Coup> = Vec::new();

        for (&square, &piece) in self.map.iter().filter( // Lets consider pieces of the player to play and the one that are not pined.
            |(_square, piece)|
            /*( !self.squares_with_pined_pieces.contains(square)) && */ (piece.color == self.player_to_play)
        ) {
            let (targetable_squares, _pieces_in_sight ) = basic_moves_for_piece_at_square(self, square);
            for &target_square in targetable_squares.iter() {
                let mover_piece = self.piece_at(square);

                if piece.kind == Pawn && (target_square.row == Row::R8 || target_square.row == Row::R1) {
                    for promot_kind in [Knight, Bishop, Tower, Queen] {
                        let coup = Coup {
                            start: square,
                            end: target_square,
                            piece: mover_piece,
                            taken: self.opt_piece_at(target_square),
                            kind: CoupKind::Promotion(promot_kind)
                        };
                        self.try_add_coup(&mut moves, coup);
                    }
                } else {
                    let coup = Coup {
                        start: square,
                        end: target_square,
                        piece: mover_piece,
                        taken: self.opt_piece_at(target_square),
                        kind: CoupKind::Normal
                    };
                    self.try_add_coup(&mut moves, coup);
                };
            }

        };


        // Add caslte if legal.
        match self.player_to_play {
            Color::White => {
                if self.white_can_h_castle { moves.push(Coup::white_h_castle()) };
                if self.white_can_a_castle { moves.push(Coup::white_a_castle()) };
            },
            Color::Black => {
                if self.black_can_h_castle { moves.push(Coup::black_h_castle()) };
                if self.black_can_a_castle { moves.push(Coup::black_a_castle()) };
            }
        }

        self.possible_moves = moves;
    }

    pub fn update_info(&mut self, level: usize) {

        if level > 0 {
            self.update_watchers_sites_and_watched_squares();
            self.update_castle_rights();
            if level > 1 {
                self.update_possible_moves();
                self.update_king_safety();
            }
        }


    }
}