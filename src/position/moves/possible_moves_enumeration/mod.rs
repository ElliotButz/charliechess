use crate::position::{
        board::types_and_structs::Board,
        color::Color::{self, *},
        coordinates::{
            converters::{row_as_square_vec, to_square_vec}, displayers::vec2str, types_and_structs::{Coords, Square}
        },
        coup::{Coup, CoupKind},
        moves::basic_piece_moves::*,
        pieces::{Piece, PieceKind::*}
    };

pub fn all_moves(board: &Board) -> (Vec<Coup>,Vec<Coup>) {
    // Returns all possible moves (aka coups) for white and black player.
    let mut white_moves: Vec<Coup> = Vec::new();
    let mut black_moves: Vec<Coup> = Vec::new(); 

    for (&square, &_piece) in board.map.iter() { 
        let (targetable_squares, _pieces_in_sight ) = basic_moves_for_piece_at_square(board, square);
        for &target_square in targetable_squares.iter() {
            let mover_piece = board.piece_at(square);
            let coup = Coup {
                start: square,
                end: target_square,
                piece: mover_piece,
                taken: board.opt_piece_at(target_square),
                kind: CoupKind::Normal
            };
            match &mover_piece.color {
                White => white_moves.push(coup),
                Black => black_moves.push(coup),
            };
        }
    };
    (white_moves, black_moves)
}

pub fn square_is_in_sight_of_opponent(board: &Board, square: Square, opponent_color: Color) -> bool {
    // Check if square is targetable by opponent piece. Checks for Pawn in a second time.

    // Check for all pieces, expect Pawn
    [Queen, Tower, Bishop, Knight, King].iter().any(|&kind|
    {
        let piece = Piece { kind: kind, color: opponent_color}; 
        let (_sighters_possible_squares, sighters) = basic_moves_for_piece_from_square(board, square, piece);
        sighters.contains(&piece)
    } )  
    ||
    { // Check for pawn
        let c_square: Coords = square.into();
        let direction_in_which_is_sighter = opponent_color.the_other().as_direction();
        to_square_vec(&vec![
            c_square + ( 1, direction_in_which_is_sighter),
            c_square + (-1, direction_in_which_is_sighter)
            ]).iter().any(|&potential_pawn_sighter_square|
            {
                match board.opt_piece_at(potential_pawn_sighter_square) {
                    Some(piece) => {
                        piece == Piece {kind: Pawn, color: opponent_color}},
                    None => false
                }
            } )
    }

}


pub fn update_castle_rights(board: &mut Board) {
    fn can_castle(
        board: &Board,
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
            square_is_in_sight_of_opponent(board, square, opponent_color)
        });

        let is_any_square_occupied = squares_in_range.iter().any(|&square| {
            board.opt_piece_at(square).is_some()
        });

        let can_castle = !(
            tower_moved ||
            king_moved ||
            is_any_square_under_threat ||
            is_any_square_occupied
        );
        can_castle
    }

    board.white_can_a_castle = can_castle(
        board,
        Color::White,
        board.a_white_tower_has_moved,
        board.white_king_has_moved,
        1,
        4
    );
    board.white_can_h_castle = can_castle(
        board,
        Color::White,
        board.h_white_tower_has_moved,
        board.white_king_has_moved,
        5,
        7
    );
    board.black_can_a_castle = can_castle(
        board,
        Color::Black,
        board.a_black_tower_has_moved,
        board.black_king_has_moved,
        1,
        4
    );
    board.black_can_h_castle = can_castle(
        board,
        Color::Black,
        board.h_black_tower_has_moved,
        board.black_king_has_moved,
        5,
        7
    );

}



