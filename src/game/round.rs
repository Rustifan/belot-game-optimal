use strum::{EnumCount, IntoEnumIterator};

use super::{
    deck::{Card, CardSuit, Deck},
    declaration::{Declaration, get_possible_declarations},
    player::{Hand, NUMBER_OF_PLAYERS, Player, Players, Team},
    trick::Trick,
};
use std::{collections::HashMap, thread::current};

#[derive(Debug, Default, Clone)]
pub struct Trump {
    pub player_index: usize,
    pub trump_suit: CardSuit,
}

impl Trump {
    fn get_caller_team(&self) -> Team {
        let team_index = self.player_index % Team::COUNT;

        Team::from_index(team_index)
    }
}

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
}

#[derive(Debug, Clone)]
pub struct TrickHistoryItem {
    trick: Trick,
    trump: Trump,
    player_index_winner: usize,
    team_winner: Team,
    points: usize,
}

impl TrickHistoryItem {
    fn new(round_state: &Round, trick: Trick) -> Self {
        let player_index_winner = trick
            .get_trick_winner(&round_state.trump.trump_suit)
            .expect("To trick is done we always have a trick winner");
        let player_winner = &round_state.players.players[player_index_winner];
        let team_winner = player_winner.get_team();
        let trump = round_state.trump.clone();
        let points = trick.get_points(&round_state.trump);

        Self {
            trick,
            trump,
            player_index_winner,
            team_winner,
            points,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct TeamPoints {
    points: [usize; Team::COUNT],
}

impl TeamPoints {
    pub fn add_points(&mut self, team: Team, points: usize) {
        let index = team.to_index();
        self.points[index] += points;
    }

    pub fn has_bigger_points(&self, team: Team) -> bool {
        let team_index = team.to_index();
        let other_team_index = (team_index + 1) % Team::COUNT;

        self.points[team_index] > self.points[other_team_index]
    }

    pub fn all_points_to_team(&mut self, team: Team) {
        let team_index = team.to_index();
        let other_team_index = (team_index + 1) % Team::COUNT;
        let points_sum = self.points.iter().fold(0, |acc, curr| acc + *curr);
        self.points[team_index] = points_sum;
        self.points[other_team_index] = 0;
    }
}

#[derive(Debug, Clone, Default)]
pub struct TeamDeclarations {
    declarations: [Vec<Declaration>; Team::COUNT],
}

impl TeamDeclarations {
    pub fn add_declaration(&mut self, player: &Player, declaration: Declaration) {
        let player_team = player.get_team();
        let team_index = player_team.to_index();
        self.declarations[team_index].push(declaration);
    }

    pub fn delete_declarations_for_team(&mut self, team: &Team) {
        let team_index = team.to_index();
        self.declarations[team_index].clear();
    }

    pub fn get_points_sum(&self, team: &Team) -> usize {
        let index = team.to_index();
        let declarations = &self.declarations[index];

        declarations.iter().fold(0, |acc, curr| curr.points + acc)
    }
}

#[derive(Debug, Clone)]
pub struct Round {
    players: Players,
    player_turn_index: usize,
    current_trick: Trick,
    trick_history: Vec<TrickHistoryItem>,
    trump: Trump,
    points: TeamPoints,
    final_points: TeamPoints,
    team_declarations: TeamDeclarations,
}

impl Round {
    pub fn new(first_player_index: usize) -> Self {
        let player_turn_index = 0;
        let mut deck = Deck::new();
        let mut players = Players::new();
        deck.shuffle_deal(&mut players);
        players.sort_hands();

        Round {
            players,
            player_turn_index: first_player_index,
            current_trick: Trick::new(player_turn_index),
            trick_history: vec![],
            trump: Trump::default(),
            points: TeamPoints::default(),
            final_points: TeamPoints::default(),
            team_declarations: TeamDeclarations::default(),
        }
    }

    pub fn get_cards_in_game(&self) -> Deck {
        let mut result = Deck::empty();
        for player in &self.players {
            result.add_hand(player.get_hand().clone());
        }

        result
    }

    fn get_trump(&mut self, round_player: &Box<dyn RoundPlayer>) -> Trump {
        let last_player_index = NUMBER_OF_PLAYERS - 1;
        for i in 0..last_player_index {
            let player_index = (i + self.player_turn_index) % NUMBER_OF_PLAYERS;
            if let Some(suit) = round_player.try_call_trump(self, player_index) {
                return Trump {
                    trump_suit: suit,
                    player_index,
                };
            }
        }

        let last_player = (last_player_index + self.player_turn_index) % NUMBER_OF_PLAYERS;
        let suit = round_player.must_call_trump(self, last_player);

        Trump {
            trump_suit: suit,
            player_index: last_player,
        }
    }

    fn play_trick(&mut self, round_player: &Box<dyn RoundPlayer>) -> TrickHistoryItem {
        while !self.current_trick.is_done() {
            let avaliable_cards = self
                .current_trick
                .get_playeble_cards(&self.players, &self.trump.trump_suit);
            let player_index = self.current_trick.get_player_index_turn();
            let played_card = round_player.play_card(&self, player_index, avaliable_cards);
            let played_card = self.players.players[player_index]
                .remove_card(&played_card)
                .expect("Player to have card that needs to be removed");
            self.current_trick.play_card(played_card);
        }
        let trick_history_item = TrickHistoryItem::new(&self, self.current_trick.clone());
        self.trick_history.push(trick_history_item.clone());
        self.increment_player_index();
        self.current_trick = Trick::new(self.player_turn_index);

        trick_history_item
    }

    pub fn play_round(&mut self, round_player: Box<dyn RoundPlayer>) {
        self.trump = self.get_trump(&round_player);
        self.team_declarations = self.get_declarations(&round_player);

        while self.players.have_cards() {
            let played_trick = self.play_trick(&round_player);
            self.points
                .add_points(played_trick.team_winner, played_trick.points);
        }
        let last_winner = &self
            .trick_history
            .last()
            .expect("trick history should have all tricks so last trick must be present")
            .team_winner;
        const LAST_WINNER_ADDITIONAL_POINTS: usize = 10;

        self.points
            .add_points(last_winner.clone(), LAST_WINNER_ADDITIONAL_POINTS);

        self.final_points = self.points.clone();
        for team in Team::iter() {
            self.final_points
                .add_points(team.clone(), self.team_declarations.get_points_sum(&team));
        }

        if self.has_trump_caller_failed() {
            self.final_points
                .all_points_to_team(self.trump.get_caller_team().get_enemy_team());
        }
    }

    fn has_trump_caller_failed(&self) -> bool {
        let player = self
            .players
            .get(self.trump.player_index)
            .expect("for player to exist with trump_player_index");
        let team = player.get_team();

        !self.final_points.has_bigger_points(team)
    }

    pub fn increment_player_index(&mut self) {
        self.player_turn_index += 1;
        self.player_turn_index %= NUMBER_OF_PLAYERS;
    }

    fn get_declarations(&self, round_player: &Box<dyn RoundPlayer>) -> TeamDeclarations {
        let mut best_declaration_result: Option<Declaration> = None;
        let mut best_declaration_player: Option<Player> = None;
        let mut team_declarations = TeamDeclarations::default();
        for index in 0..NUMBER_OF_PLAYERS {
            let player_index = (index + self.player_turn_index) % NUMBER_OF_PLAYERS;
            let player = &self.players.players[player_index];
            let possible_declarations = get_possible_declarations(&player.hand);
            let approved_declarations = possible_declarations
                .into_iter()
                .filter(|declaration| {
                    round_player.call_declaration(&self, player_index, declaration)
                })
                .collect::<Vec<_>>();

            for declaration in approved_declarations.iter() {
                if let Some(best_result) = &best_declaration_result {
                    if declaration.is_better_than(&best_result) {
                        best_declaration_result = Some(declaration.clone());
                        best_declaration_player = Some(player.clone());
                    }
                } else {
                    best_declaration_result = Some(declaration.clone());
                    best_declaration_player = Some(player.clone());
                }

                team_declarations.add_declaration(player, declaration.clone());
            }
        }
        if let Some(best_declaration_player) = best_declaration_player {
            let best_player_team = best_declaration_player.get_team();
            match best_player_team {
                Team::A => team_declarations.delete_declarations_for_team(&Team::B),
                Team::B => team_declarations.delete_declarations_for_team(&Team::A),
            };
        }

        team_declarations
    }
}
