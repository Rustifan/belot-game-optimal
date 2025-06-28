use super::{deck::{Card, CardSuit}, declaration::Declaration, round::{Round, RoundUpdateEvent}};

pub trait RoundPlayer {
    fn try_call_trump(&self, round_state: &Round, player_index: usize) -> Option<CardSuit>;
    fn must_call_trump(&self, round_state: &Round, player_index: usize) -> CardSuit;
    fn play_card(
        &self,
        round_state: &Round,
        player_index: usize,
        available_cards: Vec<Card>,
    ) -> Card;
    fn call_declaration(
        &self,
        round_state: &Round,
        player_index: usize,
        declaration: &Declaration,
    ) -> bool;
    fn will_declare_bella(&self, round_state: &Round, player_index: usize) -> bool;
    fn on_update(&self, round_state: &Round, round_event: RoundUpdateEvent);
}
