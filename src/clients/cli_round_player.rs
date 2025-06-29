use crate::game::deck::{Card, CardSuit, CardValue};
use crate::game::player::Player;
use crate::game::round::{Round, RoundUpdateEvent};
use crate::game::round_player::RoundPlayer;
use crate::utils::console::{clear_console, print_current_points, wait_for_std_input};
use crate::utils::random::get_random_suit;
use rand::seq::SliceRandom;
use std::io;

pub struct CliRoundPlayer;

impl CliRoundPlayer {
    pub fn boxed() -> Box<dyn RoundPlayer> {
        Box::new(Self)
    }

    fn is_human_player(&self, player_index: usize) -> bool {
        player_index == 0
    }

    fn print_player_cards(&self, player: &Player) {
        println!("Your cards are:");
        for (i, card) in player.hand.cards().iter().enumerate() {
            let card_suit: &str = card.suit.clone().into();
            let card_value: &str = card.value.clone().into();
            println!("{}. {} {}", i + 1, card_suit, card_value);
        }
    }

    fn print_player_hand_for_card_play(&self, player: &Player, available_cards: &[Card]) {
        println!("Your cards are:");
        for (i, card) in player.hand.cards().iter().enumerate() {
            let is_available = available_cards.contains(card);
            let marker = if is_available { "*" } else { " " };
            let card_suit: &str = card.suit.clone().into();
            let card_value: &str = card.value.clone().into();
            println!("{}.{} {} {}", i + 1, marker, card_suit, card_value);
        }
    }

    fn prompt_for_card_selection(
        &self,
        player_hand_cards: &[Card],
        available_cards: &[Card],
    ) -> Card {
        loop {
            println!("Please select a card to play (choose a number for a card marked with '*'):");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim().parse::<usize>() {
                Ok(n) if n > 0 && n <= player_hand_cards.len() => {
                    let selected_card = &player_hand_cards[n - 1];
                    if available_cards.contains(selected_card) {
                        return selected_card.clone();
                    } else {
                        println!("You cannot play that card. Please select one of the available cards.");
                    }
                }
                _ => {
                    println!(
                        "Invalid input. Please enter a number between 1 and {}.",
                        player_hand_cards.len()
                    );
                }
            }
        }
    }
}

impl RoundPlayer for CliRoundPlayer {
    fn on_update(&self, round_state: &Round, round_event: RoundUpdateEvent) {
        match round_event {
            RoundUpdateEvent::CardPlayed { player_index, card } => {
                let player = round_state.get_player_by_index(player_index);
                let card_suit: &str = card.suit.clone().into();
                let card_value: &str = card.value.clone().into();
                println!("Player {} played {} {}", player.name, card_suit, card_value);
                wait_for_std_input();
            }
            RoundUpdateEvent::DeclarationsCalled(declarations) => {
                for declaration in declarations {
                    let player_index = declaration.player_index;
                    let declaration = &declaration.declaration;
                    let player = round_state.get_player_by_index(player_index);
                    println!("{} declared: ", player.name);
                    println!("{:#?}", declaration);
                    println!("");
                }
                wait_for_std_input();
                clear_console();
                print_current_points(round_state);
            }
            RoundUpdateEvent::TrumpCallEvent { player_index, trump } => {
                let player = round_state.get_player_by_index(player_index);
                let color: &str = match trump {
                    Some(trump) => trump.trump_suit.clone().into(),
                    None => "Dalje",
                };
                println!("Player {} zove {}", player.name, color);
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

    fn try_call_trump(&self, round_state: &Round, player_index: usize) -> Option<CardSuit> {
        if self.is_human_player(player_index) {
            let player = round_state.get_player_by_index(player_index);
            let mut hand_clone = player.hand.cards().clone();
            hand_clone.shuffle(&mut rand::thread_rng());

            let (hidden_cards, shown_cards) = hand_clone.split_at(2);

            let mut sorted_shown_cards = shown_cards.to_vec();
            sorted_shown_cards.sort();

            println!("Your cards (two are hidden):");
            for (i, card) in sorted_shown_cards.iter().enumerate() {
                let card_suit: &str = card.suit.clone().into();
                let card_value: &str = card.value.clone().into();
                println!("{}. {} {}", i + 1, card_suit, card_value);
            }

            loop {
                println!("Please choose a trump suit (Leaf, Pumpkin, Herz, Acorn), or type 'Dalje' to pass:");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                let trump = match input.trim().to_lowercase().as_str() {
                    "leaf" => Some(CardSuit::Leaf),
                    "pumpkin" => Some(CardSuit::Pumpkin),
                    "herz" => Some(CardSuit::Herz),
                    "acorn" => Some(CardSuit::Acorn),
                    "dalje" => None,
                    _ => {
                        println!("Invalid input. Please try again.");
                        continue;
                    }
                };

                println!("Your hidden cards were:");
                for card in hidden_cards {
                    let card_suit: &str = card.suit.clone().into();
                    let card_value: &str = card.value.clone().into();
                    println!("{} {}", card_suit, card_value);
                }
                wait_for_std_input();

                return trump;
            }
        } else {
            if rand::random::<bool>() {
                Some(get_random_suit())
            } else {
                None
            }
        }
    }

    fn must_call_trump(&self, round_state: &Round, player_index: usize) -> CardSuit {
        if self.is_human_player(player_index) {
            let player = round_state.get_player_by_index(player_index);
            self.print_player_cards(player);
            loop {
                println!("You must choose a trump suit (Leaf, Pumpkin, Herz, Acorn):");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                match input.trim().to_lowercase().as_str() {
                    "leaf" => return CardSuit::Leaf,
                    "pumpkin" => return CardSuit::Pumpkin,
                    "herz" => return CardSuit::Herz,
                    "acorn" => return CardSuit::Acorn,
                    _ => println!("Invalid trump suit. Please try again."),
                }
            }
        } else {
            get_random_suit()
        }
    }

    fn play_card(
        &self,
        round_state: &Round,
        player_index: usize,
        available_cards: Vec<Card>,
    ) -> Card {
        if self.is_human_player(player_index) {
            let player = round_state.get_player_by_index(player_index);
            self.print_player_hand_for_card_play(player, &available_cards);
            self.prompt_for_card_selection(player.hand.cards(), &available_cards)
        } else {
            available_cards[0].clone()
        }
    }

    fn call_declaration(
        &self,
        _round_state: &Round,
        player_index: usize,
        declaration: &crate::game::declaration::Declaration,
    ) -> bool {
        if self.is_human_player(player_index) {
            println!("You have a declaration: {:#?}", declaration);
            println!("Do you want to declare it? (y/n)");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            input.trim().to_lowercase() == "y"
        } else {
            true
        }
    }

    fn will_declare_bella(&self, _round_state: &Round, player_index: usize) -> bool {
        if self.is_human_player(player_index) {
            println!("Do you want to declare bela? (y/n)");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            input.trim().to_lowercase() == "y"
        } else {
            true
        }
    }
}
