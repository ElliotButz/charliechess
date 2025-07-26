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

pub fn all_moves(board: &mut Board) -> (Vec<Coup>,Vec<Coup>) {
    // Returns all possible moves (aka coups) for white and black player.

    board.update_info();

    let mut white_moves: Vec<Coup> = Vec::new();
    let mut black_moves: Vec<Coup> = Vec::new(); 

    for (&square, &_piece) in board.map.iter().filter(
        |(square, _piece)| !board.squares_with_pined_pieces.contains(square) 
    ) { 
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

    if board.black_can_h_castle { black_moves.push(Coup::black_h_castle()) };
    if board.black_can_a_castle { black_moves.push(Coup::black_a_castle()) };
    if board.white_can_h_castle { white_moves.push(Coup::white_h_castle()) };
    if board.white_can_a_castle { white_moves.push(Coup::white_a_castle()) };



    (white_moves, black_moves)
}

