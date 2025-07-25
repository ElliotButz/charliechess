use std::collections::HashMap;

use crate::{position::{
    board::types_and_structs::Board, color::Color::*, coordinates::{converters::to_square_vec, types_and_structs::{Square, SquareVec}}, coup::Coup, moves::basic_piece_moves::*, pieces::{Piece, PieceKind::Tower}

}, square};
pub fn all_moves(board: &Board) -> Vec<Coup> {
    // Returns all possible moves (aka coups) for both player.
    let mut moves: Vec<Coup> = Vec::new(); 
    for (&square, &_piece) in board.map.iter() { 
        let (targetable_squares, _pieces_in_sight ) = basic_moves_for_piece_at_square(board, square);
        for &target_square in targetable_squares.iter() { 
            moves.push( 
                Coup {
                    start: square,
                    end: target_square,
                    piece: board.piece_at(square),
                    taken: board.opt_piece_at(target_square)
                }
            )
        }
    };
    moves
}

pub fn threatened_squares(board: &Board) -> SquareVec {
    // Return all threatened squares, tipically to determine castle rights. 
    all_moves(board).iter().map(|coup| coup.end).collect()
}

pub fn castle_rights(board: &Board) -> HashMap<Square, bool> {
    let castlers_towers_squares:SquareVec = to_square_vec(&vec![
        (1,1), (1,8), (8,1), (8,8)
    ]);
    let mut rights: HashMap<Square, bool> = HashMap::with_capacity(4);
    let threatened: SquareVec = threatened_squares(board);

    rights
}
