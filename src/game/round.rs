use super::{
    deck::{CardSuit, Deck},
    player::{Hand, NUMBER_OF_PLAYERS, Player, Players, Team},
    trick::Trick,
};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Trump {
    pub player_index: usize,
    pub trump_suit: CardSuit,
}

pub trait RoundPlayer: std::fmt::Debug {
    fn try_call_trump(&self, round_state: &Round, player_index: usize) -> Option<CardSuit>;
    fn must_call_trump(&self, round_state: &Round, player_index: usize) -> CardSuit;
}

#[derive(Debug)]
pub struct Round {
    players: Players,
    pots: HashMap<Team, Hand>,
    player_turn_index: usize,
    current_trick: Trick,
    trick_history: Vec<Trick>,
    trump: Trump,
    round_player: Box<dyn RoundPlayer>,
}

impl Round {
    pub fn new(round_player: Box<dyn RoundPlayer>, first_player_index: usize) -> Self {
        let player_turn_index = 0;
        let mut deck = Deck::new();
        let mut players = Players::new();
        deck.shuffle_deal(&mut players);
        players.sort_hands();

        Round {
            players,
            pots: HashMap::new(),
            player_turn_index: first_player_index,
            current_trick: Trick::new(player_turn_index),
            trick_history: vec![],
            trump: Trump::default(),
            round_player,
        }
    }

    pub fn get_cards_in_game(&self) -> Deck {
        let mut result = Deck::empty();
        for player in &self.players {
            result.add_hand(player.get_hand().clone());
        }

        result
    }

    fn get_trump(&mut self) -> Trump {
        let last_player_index = NUMBER_OF_PLAYERS - 1;
        for i in 0..last_player_index {
            let player_index = (i + self.player_turn_index) % NUMBER_OF_PLAYERS;
            if let Some(suit) = self.round_player.try_call_trump(self, player_index) {
                return Trump {
                    trump_suit: suit,
                    player_index,
                };
            }
        }

        let last_player = (last_player_index + self.player_turn_index) % NUMBER_OF_PLAYERS;
        let suit = self.round_player.must_call_trump(self, last_player);

        Trump {
            trump_suit: suit,
            player_index: last_player
        }
    }
    
    fn play_trick(&mut self) {
        
    }

    pub fn play_round(&mut self) {
        self.trump = self.get_trump();
        // while self.players.have_cards() {
        //     self.current_trick.play_trick()
        //
        // }
        todo!();
    }
}
