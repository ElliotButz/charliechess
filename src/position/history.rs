use crate::{coords, piece};
use crate::position::color::{Color,Color::{White, Black}};
use crate::position::coordinates::{Coords, Row, Row::*, Column, Column::*};
use crate::position::board;
use crate::position::coup::{Coup, Turn};

type CoupSerie = Vec<Coup>;
type TurnSerie = Vec<Turn>;
struct History {
    coups: CoupSerie
}

impl History {

    pub fn add_coup(&mut self, coup: Coup) {
        self.coups.push(coup);
    }

    pub fn new() -> History {
        History{coups: Vec::<Coup>::new()}
    }

    pub fn nth_coup(&self, n:usize) -> Option<Coup> {
        match 1 < n && n < self.coups.len() {
            false => None,
            true => Some(self.coups[n-1].clone()),
        }
    }

    pub fn as_turn_serie(&self) -> TurnSerie {
        let mut turn_serie: TurnSerie =  TurnSerie::new();
        for n in (1..self.coups.len()).step_by(2) {
            let white_coup: Coup = self.nth_coup(n).expect("All white choup are expected to be of type Coup, found non-Coup.");
            let black_coup: Option<Coup> = self.nth_coup(n+1);
            let turn = Turn{white_coup, black_coup};
            turn_serie.push(turn)
        }
        turn_serie
            
    }
}