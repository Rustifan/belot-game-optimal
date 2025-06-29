use std::io::{self, Write, stdout};
use crate::game::{round::Round, team::Team};

pub fn wait_for_std_input() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
}

pub fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
    stdout().flush().unwrap();
}

pub fn print_current_points(round_state: &Round) {
    let declarations = &round_state.team_declarations;
    let team_a_points = round_state.points.get_points(Team::A);
    let team_b_points = round_state.points.get_points(Team::B);
    let team_a_declarations = declarations.get_points_sum(&Team::A);
    let team_b_declarations = declarations.get_points_sum(&Team::B);
    let team_a_points = team_a_points + team_a_declarations;
    let team_b_points = team_b_points + team_b_declarations;
    let trump = &round_state.trump;
    let trump_player = &round_state.get_player_by_index(trump.player_index).name;
    let tump_color: &str = trump.trump_suit.clone().into();

    println!(
        "TEAM A: {}    TEAM B: {}      TRUMP: {} - ({})",
        team_a_points, team_b_points, tump_color, trump_player
    );
    println!("");
}
