use rand::random_range;

use crate::{
    game::{
        deck::CardSuit,
        round::{Round, RoundPlayer},
    },
    utils::random::get_random_suit,
};

#[derive(Debug)]
pub struct RandomRoundPlayer;

impl RoundPlayer for RandomRoundPlayer {
    fn try_call_trump(&self, _round_state: &Round, _player_index: usize) -> Option<CardSuit> {
        if random_range(1..11) > 8 {
            return Some(get_random_suit());
        }
        None
    }

    fn must_call_trump(&self, _round_state: &Round, _player_index: usize) -> CardSuit {
        get_random_suit()
    }

    fn play_card(
        &self,
        _round_state: &Round,
        _player_index: usize,
        available_cards: Vec<crate::game::deck::Card>,
    ) -> crate::game::deck::Card {
        available_cards[0].clone()
    }

    fn call_declaration(
        &self,
        _round_state: &Round,
        _player_index: usize,
        _declaration: &crate::game::declaration::Declaration,
    ) -> bool {
        true
    }

    fn will_declare_bella(&self, _round_state: &Round, _player_index: usize) -> bool {
        true
    }
}
