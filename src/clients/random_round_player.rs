use std::{
    fmt::format,
    io::{self, Write},
};

use rand::random_range;

use crate::{
    game::{
        deck::CardSuit,
        player::Team,
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

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush().unwrap();
}

fn print_current_points(round_state: &Round) {
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
                let player = round_state.get_player_by_index(player_index);
                println!("Player {} played {card:#?}", player.name);
                wait_for_std_input();
            }
            RoundUpdateEvent::DeclarationsCalled(declarations) => {
                for declaration in declarations {
                    let player_index = declaration.player_index;
                    let declaration = &declaration.declaration;
                    let player = round_state.get_player_by_index(player_index);
                    println!("{} declared: ", player.name);
                    println!("{declaration:#?}");
                    println!("");
                }
                wait_for_std_input();
                clear_console();
                print_current_points(round_state);
            }
            RoundUpdateEvent::TrumpCallEvent {
                player_index,
                trump,
            } => {
                if player_index == round_state.player_turn_index {
                    clear_console();
                }
                let player = round_state.get_player_by_index(player_index);
                let color = match trump {
                    Some(trump) => trump.trump_suit.clone().into(),
                    None => "Dalje",
                };
                println!("Player {} zove {color}", player.name);
                wait_for_std_input();
                if let Some(_) = trump {
                    clear_console();
                    print_current_points(round_state);
                }
            }
            RoundUpdateEvent::BelaDeclared { player_index } => {
                let player = round_state.get_player_by_index(player_index);
                println!("{} called BELA!!!", player.name);
                wait_for_std_input();
            }
            RoundUpdateEvent::TrickDone(trick_item) => {
                let points = trick_item.get_points();
                let team_winner: &str = trick_item.get_winner_team().clone().into();
                let player = round_state.get_player_by_index(trick_item.get_winner_index());

                println!(
                    "{} Won trick for team {} with {} points",
                    player.name, team_winner, points
                );
                wait_for_std_input();
                clear_console();
                print_current_points(round_state);
            }
        }
    }
}
