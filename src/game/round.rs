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
pub struct Round {
    players: Players,
    pots: HashMap<Team, Vec<Hand>>,
    player_turn_index: usize,
    current_trick: Trick,
    trick_history: Vec<Trick>,
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
            pots: HashMap::new(),
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
            let pleyed_card = self.players.players[player_index]
                .remove_card(&played_card)
                .expect("Player to have card that needs to be removed");
            self.current_trick.play_card(played_card);
        }

        let trick_winner = self
            .current_trick
            .get_trick_winner(&self.trump.trump_suit)
            .expect("To trick is done we always have a trick winner");

        let player_winner = &self.players.players[trick_winner];
        let winning_team = player_winner.get_team();

        self.trick_history.push(self.current_trick.clone());
        let cards_to_pot = Hand::new(self.current_trick.cards().clone());
        let winning_team_pot = self.pots.get_mut(&winning_team);
        if let Some(pot) = winning_team_pot {
            pot.push(cards_to_pot);
        } else {
            self.pots.insert(winning_team, vec![cards_to_pot]);
        }
        self.player_turn_index += 1;
        self.current_trick = Trick::new(self.player_turn_index);
    }

    pub fn play_round(&mut self, round_player: Box<dyn RoundPlayer>) {
        self.trump = self.get_trump(&round_player);
        self.play_trick(&round_player);
        // while self.players.have_cards() {
        //     self.current_trick.play_trick()
        //
        // }
    }
}
