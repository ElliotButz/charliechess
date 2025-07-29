use std::fmt;

use crate::position::{board::types_and_structs::Board, color::Color, coordinates::displayers::vec2str, coup::Coup, history::History};

pub mod board;
pub mod color;
pub mod coordinates;
pub mod pieces;
pub mod history;
pub mod coup;
pub mod basic_piece_moves;

#[derive(Clone)]
pub struct Position {
    pub board: Board,
    pub history: History
}

#[derive(PartialEq, Eq, Debug)]
pub enum PositionState {
    Won(Color), Draw, Ongoing
}

impl Position {

    pub fn legal_moves(& self) -> Vec<Coup> {
        self.board.possible_moves.clone()
    } 

    pub fn update(&mut self, coup: Coup) -> Result<PositionState, PositionError> {
        match self.legal_moves().contains(&coup) {
            true => { 
                self.board.execute(coup); 
                self.update_history(coup);
                Ok(self.state())
            }
            false => { Err(PositionError::IllegalMove(coup)) }
        }  
    }

    pub fn state(&self) -> PositionState {
        let player = self.player_to_move();
        let moves = self.legal_moves();
        let king_is_checked = self.board.is_checked(player);

        match moves.is_empty() {
            true => {
                println!("Possible moves {}", vec2str(&moves));
                match king_is_checked {
                    true => PositionState::Won(player.the_other()),
                    false => PositionState::Draw
                }
            }
            false => match self.history.max_times_board_occured() < 3 {
                true => PositionState::Ongoing,
                false => PositionState::Draw 
            } 
        }
    }

    pub fn new() -> Self {
        Position {
            board: Board::at_start_state(),
            history: History::new()
        }
    }
    fn update_history(&mut self, coup: Coup) {
        self.history.add_coup(coup);
    }

    fn player_to_move(&self) -> Color {
        self.board.player_to_play
    }
}

#[derive(Debug)]
pub enum PositionError { IllegalMove(Coup) }

impl fmt::Display for PositionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IllegalMove(coup) => write!(f, "Illegal move: {coup}"),
        }
    }
}