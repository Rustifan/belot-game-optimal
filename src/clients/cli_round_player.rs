use crate::game::round::{Round, RoundPlayer, RoundUpdateEvent};

pub struct CliRoundPlayer;

impl RoundPlayer for CliRoundPlayer {
    fn try_call_trump(&self, round_state: &Round, player_index: usize) -> Option<crate::game::deck::CardSuit> {
        todo!()
    }

    fn must_call_trump(&self, round_state: &Round, player_index: usize) -> crate::game::deck::CardSuit {
        todo!()
    }

    fn play_card(
        &self,
        round_state: &Round,
        player_index: usize,
        available_cards: Vec<crate::game::deck::Card>,
    ) -> crate::game::deck::Card {
        todo!()
    }

    fn call_declaration(
        &self,
        round_state: &Round,
        player_index: usize,
        declaration: &crate::game::declaration::Declaration,
    ) -> bool {
        todo!()
    }

    fn will_declare_bella(&self, round_state: &Round, player_index: usize) -> bool {
        todo!()
    }

    fn on_update(&self, round_state: &Round, round_event: RoundUpdateEvent) {
        todo!()
    }
}
