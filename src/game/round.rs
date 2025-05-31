use std::collections::HashMap;
use super::{
    deck::{CardSuit, Deck},
    player::{Hand, Player, Players, Team},
    trick::Trick,
};

#[derive(Debug, Default)]
struct Trump {
    player_index: usize,
    trump_suit: CardSuit,
}

#[derive(Debug)]
pub struct Round {
    players: Players,
    pots: HashMap<Team, Hand>,
    player_turn_index: usize,
    current_trick: Trick,
    trick_history: Vec<Trick>,
    trump: Trump
}

impl std::ops::Deref for Round {
    type Target = HashMap<Team, Hand>;

    fn deref(&self) -> &Self::Target {
        &self.pots
    }
}

impl Round {
    pub fn new() -> Self {
        let player_turn_index = 0;
        let mut deck = Deck::new();
        let mut players = Players::new();
        deck.shuffle_deal(&mut players);
        players.sort_hands();

        Round {
            players,
            pots: HashMap::new(),
            player_turn_index,
            current_trick: Trick::new(player_turn_index),
            trick_history: vec![],
            trump: Trump::default() 
        }
    }

    pub fn get_cards_in_game(&self) -> Deck {
        let mut result = Deck::empty();
        for player in &self.players {
            result.add_hand(player.get_hand().clone());
        }

        result
    }

    pub fn set_trump(&mut self, player: &Player, suit: CardSuit) {
        self.trump = Trump {
            trump_suit: suit,
            player_index: player.get_index()
        }
    }
}
