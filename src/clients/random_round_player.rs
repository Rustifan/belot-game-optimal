use std::io;

use rand::random_range;

use crate::{
    game::{
        deck::CardSuit,
        round::{Round, RoundPlayer, RoundUpdateEvent},
    },
    utils::random::get_random_suit,
};

#[derive(Debug)]
pub struct RandomRoundPlayer;

fn wait_for_std_input() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
}

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
    fn on_update(&self, round_state: &Round, round_event: RoundUpdateEvent) {
        match round_event {
            RoundUpdateEvent::CardPlayed { player_index, card } => {
                let player = round_state
                    .players
                    .get(player_index)
                    .expect("player_index should be valid index");

                println!("Player {} played {card:#?}", player.name);
            }
            RoundUpdateEvent::DeclarationsCalled(declarations) => {
                println!("Declarations: {declarations:#?}");
            }
        }
        wait_for_std_input();
    }
}
