use std::collections::HashMap;

use crate::{piece, square};
use crate::position::color::Color;
use crate::position::coordinates::converters::row_as_square_vec;
use crate::position::coup::{Coup, CoupKind::*};
use crate::position::coordinates::types_and_structs::{Column, Square};
use crate::position::pieces::{Piece, PieceKind, PieceKind::{Queen, King, Bishop, Tower}};
use crate::position::board::types_and_structs::Board;

impl Board { // Editors

    fn extract_piece_of_square(&mut self, square: Square) -> Piece {
        // Remove a Piece from a square and return it.
        self.extract_optionnal_piece_of_square(square).expect("Tried to extract Piece from an emtpy square in boardmap")
    }

    fn extract_optionnal_piece_of_square(&mut self, square: Square) -> Option<Piece> {
        // Remove an Option<Piece> from a square and return it. It will be None if no Piece was on the square.
        self.map.remove(&square)
    }

    fn add_piece_at_coords(&mut self,  coords: Square, piece: Piece) {
        self.map.insert(coords, piece);
    }

    fn can_castle(
            &self,
            king_color: Color,
            tower_moved: bool,
            king_moved: bool,
            left_bound: usize,
            right_bound: usize,
        ) -> bool {
            let row_idx: i8 = match king_color {
                Color::Black => 8,
                Color::White => 1,
            };
            let row_vec = row_as_square_vec(row_idx);
            let squares_in_range = &row_vec[left_bound..right_bound];

            let opponent_color = king_color.the_other();
            let is_any_square_under_threat = squares_in_range.iter().any(|&square| {
                self.square_is_in_sight_of_opponent(square, opponent_color)
            });

            let is_any_square_occupied = squares_in_range.iter().any(|&square| {
                self.opt_piece_at(square).is_some()
            });

            let can_castle = !(
                tower_moved ||
                king_moved ||
                is_any_square_under_threat ||
                is_any_square_occupied
            );
            can_castle
        }

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

        for color in [Color::White, Color::Black] {
            let king_square = self.squares_with(piece!(color, King))[0];
            let king_is_checked = self.square_is_in_sight_of_opponent(king_square, color.the_other());
            match color {
                Color::Black => self.black_king_is_checked = king_is_checked,
                Color::White => self.white_king_is_checked = king_is_checked
            }
        }
    }

    fn update_pines(&mut self) {

        let kind_steps: HashMap<PieceKind, Vec<(i8,i8)>> = HashMap::from([
            (Bishop, vec![(1,1), (1,-1), (-1,-1), (-1,1)]),
            (Tower,  vec![(0,1), ( 1,0), ( 0,-1), (-1,0)])
        ]);

        for king_color in [Color::White, Color::Black] {
            let king_square = self.squares_with(piece!(king_color, King))[0];
            for (&kind, steps) in &kind_steps {
                let opponent = king_color.the_other();
                let dangers = vec![
                    Piece { kind: kind,  color: opponent },
                    Piece { kind: Queen, color: opponent }
                ];
                for &step in steps {
                    let mut squares_and_pieces_on_way: ordered_hash_map::OrderedHashMap<Square, Piece> = self.step_through_piece(king_square, step.into());
                    if let Some((pined_square,  pined_piece )) = squares_and_pieces_on_way.pop_front_entry() {
                        if let Some((piner_square,  piner_piece )) = squares_and_pieces_on_way.pop_front_entry() {
                            if pined_piece.color == king_color && dangers.contains(&piner_piece) {
                                self.squares_with_pined_pieces .push(pined_square);
                                self.squares_with_pining_pieces.push(piner_square);
                            }
                        }
                    }
                }

            }
            
        }
    }

    pub fn update_info(&mut self) {
        self.player_to_play = self.player_to_play.the_other(); 
        self.update_king_safety();
        self.update_castle_rights();
        self.update_pines();
    }

    fn move_piece(&mut self, start_square: Square, target_square: Square) -> Option<Piece> {
    /*
    1: Extract the Piece at start_square (displaced: Piece),
    2: Extract the possible Piece at target_square (taken : Option<Piece>),
    3: Place the Piece displaced from start_square at target_square,
    */
        let mut touched_piece: Vec<Piece> = Vec::new();
        let displaced: Piece = self.extract_piece_of_square(start_square);
        let taken: Option<Piece> = self.extract_optionnal_piece_of_square(target_square);
        self.add_piece_at_coords(target_square, displaced);

        touched_piece.push(displaced);
        match taken {
            Some(piece) => { touched_piece.push(piece); },
            None => {}
        }

        for &piece in touched_piece.iter() {
            match piece.kind {
                King => match piece.color {
                    Color::White => self.white_king_has_moved = true,
                    Color::Black => self.black_king_has_moved = true
                },
                Tower => match piece.color {
                    Color::White => match start_square.col {
                        Column::H => self.h_white_tower_has_moved = true,
                        Column::A => self.a_white_tower_has_moved = true,
                        _ => {}
                    },
                    Color::Black => match start_square.col {
                        Column::H => self.h_black_tower_has_moved = true,
                        Column::A => self.a_black_tower_has_moved = true,
                        _ => {}
                    },
                },
                _ => {}
            }
        } 

        taken
    }

    pub fn execute(&mut self, coup: Coup) {
        match coup.kind {
            Normal => {
                self.move_piece(coup.start, coup.end);
            },
            Castle => {
                let row = coup.start.row;
                if coup.end.col == Column::C { // Long castle
                    self.move_piece(coup.start, square!((Column::C, row)));
                    self.move_piece(square!((Column::A, row)), square!((Column::D, row)));
                } else
                if coup.end.col == Column::G { // Short castle
                    self.move_piece(coup.start, square!((Column::G, row)));
                    self.move_piece(square!((Column::H, row)), square!((Column::F, row)));
                }
            }
        }
        self.last_move = coup;
        self.update_info();
    }

}