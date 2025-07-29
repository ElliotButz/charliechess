use std::{thread, time};


use crate::{
    player::structs_and_traits::{Player},
    position::{ color::Color, Position, PositionState 
    }};

pub struct Game {
    position: Position,
    black: Player,
    white: Player,
}

impl Game {
    pub fn new(white: Player, black: Player) -> Self {
        Game {
            position: Position::new(),
            black: black,
            white: white
        }
    }

    pub fn cli_play(&mut self, max_turn: usize, show_progress: bool) -> Position {
        while
        (self.position.state() == PositionState::Ongoing) &&
        (self.position.history.n_turns() < max_turn) {

            let mut position = self.position.clone();
            

            let next_coup =  loop { // Ask players for coup until they give a correct one.

                let side_to_play = self.position.board.player_to_play;
                let next_coup = match side_to_play {
                    Color::White => (self.white.strategy)(&mut position),
                    Color::Black => (self.black.strategy)(&mut position) 
                };

                if self.position.legal_moves().contains(&next_coup) { break next_coup }
                else { println!("{} proposed invalid move {} for board:\n {}", side_to_play, next_coup, position.board) }
            } ;

            match self.position.update(next_coup) {
                Ok(_) => {
                    if show_progress {
                    println!("{}", self.position.board);
                    thread::sleep(time::Duration::from_millis(10))
                    }
                },
                Err(error) => { println!("{error}") }
            }
        }
        return self.position.clone()
    }
}