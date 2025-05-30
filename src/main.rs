mod game;
use game::{deck::Deck, player::Players};

fn main() {
    let mut deck = Deck::new();
    let mut players = Players::new();
    deck.shuffle_deal(&mut players);
    players.sort_hands();
    println!("{players:#?}");
}
