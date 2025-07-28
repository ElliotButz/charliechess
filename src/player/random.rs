use rand::prelude::IndexedRandom;

use crate::{player::structs_and_traits::Player, position::{coup::Coup, Position}};

pub fn random_pick(for_pos: &mut Position) -> Coup {
        let mut rng = rand::rng();
        for_pos.legal_moves().choose(&mut rng).cloned().expect("No available move !")
    }
impl Player {
    pub fn random() -> Self {
        Player { strategy : random_pick }
    }
}
