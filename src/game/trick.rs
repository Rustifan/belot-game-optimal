use std::collections::HashSet;

use crate::game::points::{better_than_normal, better_than_trump};

use super::{
    deck::{Card, CardSuit},
    player::Players,
    points::{get_best_normal, get_best_trump},
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

#[cfg(test)]
mod trick_tests {
    use std::collections::HashSet;

    use crate::game::player::{Hand, Player, Players};
    use crate::game::{deck::Card, trick::Trick};
    use crate::game::{deck::CardSuit, deck::CardValue};

    #[test]
    fn test_get_playeble_cards_empty_table() {
        let mut trick = Trick::new(3);
        trick.cards_on_table = vec![
            Card {
                value: CardValue::IX,
                suit: CardSuit::Acorn,
            },
            Card {
                value: CardValue::X,
                suit: CardSuit::Acorn,
            },
            Card {
                value: CardValue::VIII,
                suit: CardSuit::Acorn,
            },
        ];
        let trump_color = CardSuit::Herz;
        let mut players = Players::default();
        let test_cards = vec![
            Card {
                value: CardValue::IX,
                suit: CardSuit::Leaf,
            },
            Card {
                value: CardValue::Kec,
                suit: CardSuit::Acorn,
            },
            Card {
                value: CardValue::VIII,
                suit: CardSuit::Acorn,
            },
            Card {
                value: CardValue::X,
                suit: CardSuit::Herz,
            },
        ];
        players.players[3] = Player {
            name: "test".to_string(),
            index: 3,
            hand: Hand { hand: test_cards },
        };

        let result = trick.get_playeble_cards(&players, &trump_color);
        let expected_vec = vec![Card {
            value: CardValue::Kec,
            suit: CardSuit::Acorn,
        }];
        let expected_hash = expected_vec.into_iter().collect::<HashSet<Card>>();

        assert_eq!(result, expected_hash);
    }
}
