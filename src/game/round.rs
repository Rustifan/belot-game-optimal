use super::{
    deck::{Card, CardSuit, Deck},
    player::{Hand, NUMBER_OF_PLAYERS, Player, Players, Team},
    trick::Trick,
};
use std::{collections::HashMap, thread::current};

#[derive(Debug, Default, Clone)]
pub struct Trump {
    pub player_index: usize,
    pub trump_suit: CardSuit,
}

pub trait RoundPlayer {
    fn try_call_trump(&self, round_state: &Round, player_index: usize) -> Option<CardSuit>;
    fn must_call_trump(&self, round_state: &Round, player_index: usize) -> CardSuit;
    fn play_card(
        &self,
        round_state: &Round,
        player_index: usize,
        available_cards: Vec<Card>,
    ) -> Card;
}

#[derive(Debug, Clone)]
pub struct TrickHistoryItem {
    trick: Trick,
    trump: Trump,
    player_index_winner: usize,
    team_winner: Team,
    points: usize,
}

impl TrickHistoryItem {
    fn new(round_state: &Round, trick: Trick) -> Self {
        let player_index_winner = trick
            .get_trick_winner(&round_state.trump.trump_suit)
            .expect("To trick is done we always have a trick winner");
        let player_winner = &round_state.players.players[player_index_winner];
        let team_winner = player_winner.get_team();
        let trump = round_state.trump.clone();
        let points = trick.get_points(&round_state.trump);

        Self {
            trick,
            trump,
            player_index_winner,
            team_winner,
            points,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Round {
    players: Players,
    player_turn_index: usize,
    current_trick: Trick,
    trick_history: Vec<TrickHistoryItem>,
    trump: Trump,
}

impl Round {
    pub fn new(first_player_index: usize) -> Self {
        let player_turn_index = 0;
        let mut deck = Deck::new();
        let mut players = Players::new();
        deck.shuffle_deal(&mut players);
        players.sort_hands();

        Round {
            players,
            player_turn_index: first_player_index,
            current_trick: Trick::new(player_turn_index),
            trick_history: vec![],
            trump: Trump::default(),
        }
    }

    pub fn get_cards_in_game(&self) -> Deck {
        let mut result = Deck::empty();
        for player in &self.players {
            result.add_hand(player.get_hand().clone());
        }

        result
    }

    fn get_trump(&mut self, round_player: &Box<dyn RoundPlayer>) -> Trump {
        let last_player_index = NUMBER_OF_PLAYERS - 1;
        for i in 0..last_player_index {
            let player_index = (i + self.player_turn_index) % NUMBER_OF_PLAYERS;
            if let Some(suit) = round_player.try_call_trump(self, player_index) {
                return Trump {
                    trump_suit: suit,
                    player_index,
                };
            }
        }

        let last_player = (last_player_index + self.player_turn_index) % NUMBER_OF_PLAYERS;
        let suit = round_player.must_call_trump(self, last_player);

        Trump {
            trump_suit: suit,
            player_index: last_player,
        }
    }

    fn play_trick(&mut self, round_player: &Box<dyn RoundPlayer>) {
        while !self.current_trick.is_done() {
            let avaliable_cards = self
                .current_trick
                .get_playeble_cards(&self.players, &self.trump.trump_suit);
            let player_index = self.current_trick.get_player_index_turn();
            let played_card = round_player.play_card(&self, player_index, avaliable_cards);
            let played_card = self.players.players[player_index]
                .remove_card(&played_card)
                .expect("Player to have card that needs to be removed");
            self.current_trick.play_card(played_card);
        }

        self.trick_history
            .push(TrickHistoryItem::new(&self, self.current_trick.clone()));
        self.increment_player_index();
        self.current_trick = Trick::new(self.player_turn_index);
    }

    pub fn play_round(&mut self, round_player: Box<dyn RoundPlayer>) {
        self.trump = self.get_trump(&round_player);
        while self.players.have_cards() {
            self.play_trick(&round_player);

        }
    }

    pub fn increment_player_index(&mut self){
        self.player_turn_index +=1;
        self.player_turn_index %= NUMBER_OF_PLAYERS;
    }
}
