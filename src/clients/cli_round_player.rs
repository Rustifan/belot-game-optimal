use crate::game::{round::{Round, RoundUpdateEvent}, round_player::RoundPlayer};

pub struct CliRoundPlayer;

impl RoundPlayer for CliRoundPlayer {
    fn try_call_trump(
        &self,
        _round_state: &Round,
        _player_index: usize,
    ) -> Option<crate::game::deck::CardSuit> {
        todo!()
    }

    fn must_call_trump(
        &self,
        _round_state: &Round,
        _player_index: usize,
    ) -> crate::game::deck::CardSuit {
        todo!()
    }

    fn play_card(
        &self,
        _round_state: &Round,
        _player_index: usize,
        _available_cards: Vec<crate::game::deck::Card>,
    ) -> crate::game::deck::Card {
        todo!()
    }

    fn call_declaration(
        &self,
        _round_state: &Round,
        _player_index: usize,
        _declaration: &crate::game::declaration::Declaration,
    ) -> bool {
        todo!()
    }

    fn will_declare_bella(&self, _round_state: &Round, _player_index: usize) -> bool {
        todo!()
    }

    fn on_update(&self, _round_state: &Round, _round_event: RoundUpdateEvent) {
        todo!()
    }
}
