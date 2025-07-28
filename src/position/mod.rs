use crate::position::{board::types_and_structs::Board, color::Color, coup::Coup, history::History};

pub mod board;
pub mod color;
pub mod coordinates;
pub mod pieces;
pub mod history;
pub mod coup;
pub mod basic_piece_moves;

pub struct Position {
    board: Board,
    history: History
}

impl Position {

    fn is_draw(&self) -> bool {
        todo!()
    }

    fn is_mate(&self) -> bool {
        todo!()
    }

    fn is_stale_mate(self) -> bool {
        todo!()
    }

    fn update_hitory(&mut self, coup: Coup) {
        todo!()
    }

    fn player_to_move(&self) -> Color {
        todo!()
    }
}