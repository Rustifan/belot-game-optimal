use std::collections::HashSet;

use super::{
    deck::{Card, CardSuit},
    player::Players,
};

#[derive(Debug)]
pub struct Trick {
    player_index_turn: usize,
    cards_on_table: Vec<Card>,
}

impl Trick {
    pub fn new(player_index_turn: usize) -> Self {
        Trick {
            cards_on_table: vec![],
            player_index_turn,
        }
    }

    pub fn cards(&self) -> &Vec<Card> {
        &self.cards_on_table
    }

    pub fn play_card(&mut self, card: Card) {
        self.cards_on_table.push(card);
        self.player_index_turn += 1;
    }

    pub fn get_player_index_turn(&self) -> usize {
        self.player_index_turn
    }

    pub fn into_cards(self) -> Vec<Card> {
        self.cards_on_table
    }

    pub fn get_playeble_cards(&self, players: &Players, trump_color: &CardSuit) -> HashSet<Card> {
        let number_of_cards_on_table = self.cards_on_table.len();
        let player_cards = players
            .get(self.get_player_index_turn())
            .expect("player index always to be inside player boundaries")
            .get_hand()
            .cards();
        let cloned_cards = player_cards.clone();
        if number_of_cards_on_table == 0 {
            return cloned_cards.into_iter().collect();
        }
        let filterd_cards = self.filter_by_played_first_card(cloned_cards, trump_color);

        filterd_cards.into_iter().collect()
    }

    fn filter_by_played_first_card(&self, cards: Vec<Card>, trump_color: &CardSuit) -> Vec<Card> {
        let first_card = self
            .cards_on_table
            .get(0)
            .expect("should have at least one card on table");
        let has_matching_suit = cards.iter().any(|card| card.suit == first_card.suit);
        if has_matching_suit {
            return cards
                .into_iter()
                .filter(|card| card.suit == first_card.suit)
                .collect();
        }

        let has_trump_suit = cards.iter().any(|card| card.suit == *trump_color);
        if has_trump_suit {
            return cards
                .into_iter()
                .filter(|card| card.suit == *trump_color)
                .collect();
        }

        cards
    }
}
