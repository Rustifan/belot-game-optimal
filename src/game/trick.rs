use std::collections::HashSet;

use crate::game::points::{better_than_normal, better_than_trump};

use super::{
    deck::{Card, CardSuit},
    player::{NUMBER_OF_PLAYERS, Players},
    points::{get_best_normal, get_best_trump},
};

#[derive(Debug)]
pub struct Trick {
    player_index_turn: usize,
    pub cards_on_table: Vec<Card>,
}

impl Trick {
    pub fn new(player_index_turn: usize) -> Self {
        Trick {
            cards_on_table: vec![],
            player_index_turn,
        }
    }

    pub fn is_done(&self) -> bool {
        self.cards_on_table.len() >= NUMBER_OF_PLAYERS
    }

    pub fn cards(&self) -> &Vec<Card> {
        &self.cards_on_table
    }

    fn trick_winner_by_color(
        &self,
        color: &CardSuit,
        better: fn(a: &Card, b: &Card) -> bool,
    ) -> Option<usize> {
        let best_card_index_on_table = self
            .cards_on_table
            .iter()
            .enumerate()
            .filter(|(_, card)| card.suit == *color)
            .reduce(|acc, curr| if better(&acc.1, &curr.1) { acc } else { curr })?
            .0;
        let player_index = (best_card_index_on_table + self.player_index_turn) % NUMBER_OF_PLAYERS;

        return Some(player_index);
    }

    pub fn get_trick_winner(&self, trump: &CardSuit) -> Option<usize> {
        if !self.is_done() {
            return None;
        }

        let has_trump = self.cards_on_table.iter().any(|card| card.suit == *trump);
        if has_trump {
            return self.trick_winner_by_color(trump, better_than_trump);
        }
        let first_card_color = &self.cards_on_table.get(0)?.suit;

        return self.trick_winner_by_color(first_card_color, better_than_normal);
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
        let best_trump = get_best_trump(&self.cards_on_table, trump_color.clone());
        let best_normal = get_best_normal(&self.cards_on_table);
        // path where we have same suit and suit is not trump
        if has_matching_suit && *trump_color != first_card.suit {
            return self.filter_normal_options(
                cards,
                best_trump,
                best_normal.expect("to have at least one card"),
                first_card,
            );
        }
        // if we have trump suit
        let has_trump_suit = cards.iter().any(|card| card.suit == *trump_color);
        if has_trump_suit {
            return self.filter_trump_options(cards, best_trump, trump_color);
        }

        cards
    }

    fn filter_normal_options(
        &self,
        cards: Vec<Card>,
        best_trump: Option<Card>,
        best_normal: Card,
        first_card: &Card,
    ) -> Vec<Card> {
        let filtered: Vec<Card> = cards
            .into_iter()
            .filter(|card| card.suit == first_card.suit)
            .collect();

        if let Some(_) = best_trump {
            return filtered;
        }

        let has_uber = filtered
            .iter()
            .any(|card| better_than_normal(&card, &best_normal));
        if !has_uber {
            return filtered;
        }

        filtered
            .into_iter()
            .filter(|card| better_than_normal(&card, &best_normal))
            .collect()
    }

    fn filter_trump_options(
        &self,
        cards: Vec<Card>,
        best_trump: Option<Card>,
        trump_color: &CardSuit,
    ) -> Vec<Card> {
        let filtered: Vec<Card> = cards
            .into_iter()
            .filter(|card| card.suit == *trump_color)
            .collect();
        if let None = best_trump {
            return filtered;
        }
        let best_trump = best_trump.expect("must be Some because we checked None");
        let has_uber = filtered
            .iter()
            .any(|card| better_than_trump(card, &best_trump));
        if !has_uber {
            return filtered;
        }

        filtered
            .into_iter()
            .filter(|card| better_than_trump(card, &best_trump))
            .collect()
    }
}
