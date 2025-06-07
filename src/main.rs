mod clients;
mod game;
mod utils;
use game::round::Round;
use clients::random_round_player::RandomRoundPlayer;

fn main() {
    let round_player = Box::from(RandomRoundPlayer);
    let mut round = Round::new(0); 
    round.play_round(round_player);

    println!("{round:#?}");
}
