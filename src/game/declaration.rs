use crate::game::player::Player;
use std::collections::HashMap;

use strum::{EnumCount, IntoEnumIterator};

use super::{
    deck::{Card, CardSuit, CardValue},
    player::Hand, team::Team,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Declaration {
    pub points: usize,
    pub cards: Vec<Card>,
}

impl Declaration {
    pub fn is_better_than(&self, other: &Declaration) -> bool {
        if self.points == other.points {
            return self.cards.len() < other.cards.len();
        }

        self.points > other.points
    }
}

fn get_scale_index(card_value: &CardValue) -> usize {
    match card_value {
        CardValue::VII => 0,
        CardValue::VIII => 1,
        CardValue::IX => 2,
        CardValue::X => 3,
        CardValue::Jack => 4,
        CardValue::Queen => 5,
        CardValue::King => 6,
        CardValue::Kec => 7,
    }
}
fn get_scale_points(scale_len: usize) -> usize {
    match scale_len {
        3 => 20,
        4 => 50,
        5..=7 => 100,
        8 => 1000,
        _ => 0,
    }
}

fn get_points_from_four_of_a_kind(card_value: &CardValue) -> usize {
    match card_value {
        CardValue::VII => 0,
        CardValue::VIII => 0,
        CardValue::IX => 150,
        CardValue::X => 100,
        CardValue::Jack => 200,
        CardValue::Queen => 100,
        CardValue::King => 100,
        CardValue::Kec => 100,
    }
}

fn get_four_of_a_kind_declarations(hand: &Hand) -> Vec<Declaration> {
    let mut hash_counter: HashMap<CardValue, usize> = HashMap::new();
    let mut declarations_result: Vec<Declaration> = vec![];
    for card in hand.cards().iter() {
        let counter = hash_counter.get_mut(&card.value);
        if let Some(counter) = counter {
            *counter += 1;
        } else {
            hash_counter.insert(card.value.clone(), 1);
        }
    }
    for (value, _) in hash_counter.into_iter().filter(|(_, size)| *size == 4) {
        let points = get_points_from_four_of_a_kind(&value);
        if points == 0 {
            continue;
        }
        let mut cards: Vec<Card> = vec![];
        for card in hand.cards().iter() {
            if card.value != value {
                continue;
            }
            cards.push(card.to_owned());
        }

        cards.sort_by_key(|card| card.suit.clone());
        let declaration = Declaration { points, cards };
        declarations_result.push(declaration);
    }

    declarations_result
}

fn get_scales_by_suit(suit: &CardSuit, hand: &Hand) -> Vec<Declaration> {
    let mut result_declarations: Vec<Declaration> = vec![];
    let mut cards = hand
        .cards()
        .into_iter()
        .filter(|card| card.suit == *suit)
        .collect::<Vec<_>>();
    cards.sort_by_key(|card| get_scale_index(&card.value));
    let mut i = 0;
    while i < cards.len() {
        let mut card_val = get_scale_index(&cards[i].value);
        let mut j = i + 1;
        while j < cards.len() {
            let j_card = get_scale_index(&cards[j].value);
            let j_card_should_be = card_val + 1;
            if j_card != j_card_should_be {
                break;
            }
            card_val = get_scale_index(&cards[j].value);
            j += 1;
        }
        let in_row = j - i;
        let points = get_scale_points(in_row);
        if points == 0 {
            i = j;
            continue;
        }

        let declaration_cards = cards[i..j]
            .to_vec()
            .into_iter()
            .map(|card| card.clone())
            .collect::<Vec<_>>();
        let declaration = Declaration {
            points,
            cards: declaration_cards,
        };

        result_declarations.push(declaration);
        i = j;
    }

    result_declarations
}

fn get_scale_declarations(hand: &Hand) -> Vec<Declaration> {
    let mut result_declarations: Vec<Declaration> = vec![];
    for suit in CardSuit::iter() {
        let declarations = get_scales_by_suit(&suit, hand);
        result_declarations.extend(declarations);
    }
    result_declarations
}

pub fn get_possible_declarations(hand: &Hand) -> Vec<Declaration> {
    let scale_declarations = get_scale_declarations(hand);
    let mut four_of_a_kind_declarations = get_four_of_a_kind_declarations(hand);
    four_of_a_kind_declarations.sort_by_key(|declaration| declaration.points);

    scale_declarations
        .into_iter()
        .chain(four_of_a_kind_declarations.into_iter())
        .collect()
}

#[derive(Debug, Clone, Default)]
pub struct TeamDeclarations {
    pub declarations: [Vec<DeclaratonWithPlayerInfo>; Team::COUNT],
}

#[derive(Debug, Clone)]
pub struct DeclaratonWithPlayerInfo {
    pub declaration: Declaration,
    pub player_index: usize,
}

impl TeamDeclarations {
    pub fn add_declaration(&mut self, player: &Player, declaration: Declaration) {
        let player_team = player.get_team();
        let team_index = player_team.to_index();
        let declaraiton_with_player_info = DeclaratonWithPlayerInfo {
            declaration,
            player_index: player.get_index(),
        };
        self.declarations[team_index].push(declaraiton_with_player_info);
    }

    pub fn delete_declarations_for_team(&mut self, team: &Team) {
        let team_index = team.to_index();
        self.declarations[team_index].clear();
    }

    pub fn get_points_sum(&self, team: &Team) -> usize {
        let index = team.to_index();
        let declarations = &self.declarations[index];

        declarations
            .iter()
            .fold(0, |acc, curr| curr.declaration.points + acc)
    }
}
