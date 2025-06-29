use strum::EnumCount;

use super::{deck::CardSuit, player::Team};


#[derive(Debug, Default, Clone)]
pub struct Trump {
    pub player_index: usize,
    pub trump_suit: CardSuit,
}

impl Trump {
    pub fn get_caller_team(&self) -> Team {
        let team_index = self.player_index % Team::COUNT;

        Team::from_index(team_index)
    }
}
