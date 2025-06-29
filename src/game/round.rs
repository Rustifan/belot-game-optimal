use strum::IntoEnumIterator;

use crate::game::team::Team;
use crate::game::declaration::DeclaratonWithPlayerInfo;
use crate::game::round_player::RoundPlayer;

use super::{
    deck::{Card, Deck}, declaration::{get_possible_declarations, Declaration, TeamDeclarations}, player::{Player, Players, NUMBER_OF_PLAYERS}, team::TeamPoints, trick::{Trick, TrickHistoryItem}, trump::Trump 
};

pub enum RoundUpdateEvent<'a> {
    CardPlayed {
        player_index: usize,
        card: Card,
    },
    TrumpCallEvent {
        player_index: usize,
        trump: Option<&'a Trump>,
    },
    DeclarationsCalled(&'a Vec<DeclaratonWithPlayerInfo>),
    BelaDeclared {
        player_index: usize,
    },
    TrickDone(TrickHistoryItem),
}

#[derive(Debug, Clone)]
pub struct Round {
    pub players: Players,
    pub player_turn_index: usize,
    pub current_trick: Trick,
    pub trick_history: Vec<TrickHistoryItem>,
    pub trump: Trump,
    pub points: TeamPoints,
    pub final_points: TeamPoints,
    pub team_declarations: TeamDeclarations,
    pub bela_declared: Option<Team>,
}

impl Round {
    pub fn new(first_player_index: usize, player_names: [&'static str; NUMBER_OF_PLAYERS]) -> Self {
        let player_turn_index = 0;
        let mut deck = Deck::new();
        let mut players = Players::new(player_names);
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
            bela_declared: None,
        }
    }

    pub fn get_player_by_index(&self, player_index: usize) -> &Player {
        self.players
            .get(player_index)
            .expect("player_index should be valid index")
    }

    fn is_stigl(&self) -> Option<Team> {
        let trick_history = &self.trick_history;
        let team_a_trick_count: usize = trick_history.into_iter().fold(0, |acc, curr| {
            if curr.team_winner == Team::A {
                acc + 1
            } else {
                acc
            }
        });

        const ALL_TRICKS_COUNT: usize = 8;
        const NONE_TRICKS_COUNT: usize = 0;

        match team_a_trick_count {
            ALL_TRICKS_COUNT => Some(Team::A),
            NONE_TRICKS_COUNT => Some(Team::B),
            _ => None,
        }
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

            round_player.on_update(
                &self,
                RoundUpdateEvent::TrumpCallEvent {
                    player_index,
                    trump: None,
                },
            );
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
            let player = &mut self.players.players[player_index];
            let has_bela = player.hand.has_bela(&self.trump);
            let played_card = player
                .remove_card(&played_card)
                .expect("Player to have card that needs to be removed");
            if has_bela
                && played_card.is_bela_card(&self.trump)
                && round_player.will_declare_bella(&self, player_index)
            {
                self.bela_declared = Some(Team::from_player_index(player_index));
                let bela_event = RoundUpdateEvent::BelaDeclared { player_index };
                round_player.on_update(&self, bela_event);
            }

            self.current_trick.play_card(played_card.clone());
            round_player.on_update(
                &self,
                RoundUpdateEvent::CardPlayed {
                    card: played_card,
                    player_index,
                },
            );
        }
        let trick_history_item = TrickHistoryItem::new(&self, self.current_trick.clone());
        self.trick_history.push(trick_history_item.clone());
        self.increment_player_index();
        self.current_trick = Trick::new(self.player_turn_index);

        trick_history_item
    }

    fn try_publish_declaration_event(&self, round_player: &Box<dyn RoundPlayer>) {
        for team_declaration in self
            .team_declarations
            .declarations
            .iter()
            .filter(|vec| vec.len() > 0)
        {
            let round_event = RoundUpdateEvent::DeclarationsCalled(&team_declaration);
            round_player.on_update(self, round_event);
        }
    }

    pub fn play_round(&mut self, round_player: Box<dyn RoundPlayer>) {
        self.trump = self.get_trump(&round_player);
        let trump_event = RoundUpdateEvent::TrumpCallEvent {
            player_index: self.trump.player_index,
            trump: Some(&self.trump),
        };
        round_player.on_update(&self, trump_event);

        self.team_declarations = self.get_declarations(&round_player);
        self.try_publish_declaration_event(&round_player);

        while self.players.have_cards() {
            let played_trick = self.play_trick(&round_player);
            self.points
                .add_points(played_trick.team_winner, played_trick.points);
            round_player.on_update(&self, RoundUpdateEvent::TrickDone(played_trick));
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

        if let Some(bela_team) = self.bela_declared {
            const BELA_POINTS: usize = 20;
            self.final_points.add_points(bela_team, BELA_POINTS);
        }

        if let Some(stigl_team) = self.is_stigl() {
            const STIGL_POINTS: usize = 90;
            self.final_points.add_points(stigl_team, STIGL_POINTS);
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
