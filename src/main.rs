mod game;
use game::{round::Round, test_round_player::TestRoundPlayer};

fn main() {
    let round_player = Box::from(TestRoundPlayer);
    let mut round = Round::new(round_player, 0);
    round.play_round();
    
    println!("{round:#?}");
}
