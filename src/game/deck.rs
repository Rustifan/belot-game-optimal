use super::player::Players;
use crate::game::trump::Trump;
use rand::random_range;
use strum::IntoEnumIterator;
use strum_macros::{EnumCount, EnumIter, IntoStaticStr};

#[derive(
    Debug, EnumIter, Clone, EnumCount, PartialEq, PartialOrd, Eq, Ord, Hash, IntoStaticStr,
)]
pub enum CardValue {
    VII,
    VIII,
    IX,
    X,
    Jack,
    Queen,
    King,
    Kec,
}

#[derive(
    Debug, IntoStaticStr, EnumIter, Clone, EnumCount, PartialEq, PartialOrd, Eq, Ord, Default, Hash,
)]
pub enum CardSuit {
    #[default]
    Leaf,
    Pumpkin,
    Herz,
    Acorn,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Card {
    pub value: CardValue,
    pub suit: CardSuit,
}

impl Card {
    pub fn new(suit: CardSuit, value: CardValue) -> Self {
        Self { suit, value }
    }

    pub fn is_bela_card(&self, trump: &Trump) -> bool {
        if self.suit != trump.trump_suit {
            return false;
        }

        self.value == CardValue::Queen || self.value == CardValue::King
    }
}

#[derive(Debug)]
pub struct Deck {
    pub deck: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut deck: Vec<Card> = vec![];
        for suit in CardSuit::iter() {
            for value in CardValue::iter() {
                let card = Card::new(suit.clone(), value);
                deck.push(card);
            }
        }

        Self { deck }
    }

    fn deal_card(&mut self) -> Option<Card> {
        let len = self.deck.len();
        if len == 0 {
            return None;
        }
        if len == 1 {
            let card = self.deck.pop().expect("should be 1 card in deck");
            return Some(card);
        }
        let random_index = random_range(0..len);
        let card = self.deck[random_index].clone();
        self.deck[random_index] = self.deck.last().unwrap().clone();
        self.deck.pop().expect("should be at least 1 card in deck");
        Some(card)
    }

    pub fn shuffle_deal(&mut self, players: &mut Players) {
        while self.deck.len() > 0 {
            let card = self.deal_card();
            if let Some(card) = card {
                players.give_card_to_next_player(card);
            }
        }
    }
}

