use crate::{position::{board::types_and_structs::Board, coordinates::types_and_structs::Column, coup::{Coup, CoupKind::*}, history::History}, square};

pub mod board;
pub mod color;
pub mod coordinates;
pub mod pieces;
pub mod history;
pub mod coup;
pub mod moves;

pub struct Position {
    board: Board,
    history: History
}

impl Position {
    pub fn execute(&mut self, coup: Coup) {
        match coup.kind {
            Normal => { self.board.move_piece(coup.start, coup.end); },
            Castle => {
                let row = coup.start.row;
                if coup.end.col == Column::C { // Long castle
                    self.board.move_piece(coup.start, square!((Column::C, row)));
                    self.board.move_piece(square!((Column::A, row)), square!((Column::D, row)));
                } else
                if coup.end.col == Column::G { // Short castle
                    self.board.move_piece(coup.start, square!((Column::G, row)));
                    self.board.move_piece(square!((Column::H, row)), square!((Column::F, row)));
                }
            }
        }
        
        self.board.last_move = coup;
        self.board.update_info();

    }
}