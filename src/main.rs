mod clients;
mod game;
mod utils;
use clients::cli_round_player::CliRoundPlayer;
use game::{round::Round};

fn main() {
    let round_player = CliRoundPlayer::boxed();
    let test_player_names = ["Beki", "Zvona", "Murko", "Zorka"];
    let mut round = Round::new(0, test_player_names);
    round.play_round(round_player);

    println!("points {:#?}", round.points);
    println!("final_points {:#?}", round.final_points);
}
