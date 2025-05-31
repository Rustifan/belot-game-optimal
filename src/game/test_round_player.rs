use rand::random_range;
use strum::EnumCount;

use super::{deck::CardSuit, round::RoundPlayer};

#[derive(Debug)]
pub struct TestRoundPlayer;

fn get_random_suit()->CardSuit{
    let number = random_range(0..CardSuit::COUNT);
    match number {
        0 => CardSuit::Leaf,
        1 => CardSuit::Pumpkin, 
        2 => CardSuit::Herz,
        3 => CardSuit::Acorn,
        _ => CardSuit::Leaf, // Fallback to Leaf if out of range
    }
}

impl RoundPlayer for TestRoundPlayer {
    fn try_call_trump(
        &self,
        _round_state: &super::round::Round,
        _player_index: usize,
    ) -> Option<CardSuit> {
        if random_range(1..11) > 8 {
            return Some(get_random_suit());
        }
        None
    }

    fn must_call_trump(
        &self,
        _round_state: &super::round::Round,
        _player_index: usize,
    ) -> CardSuit {
        get_random_suit()
    }
}
