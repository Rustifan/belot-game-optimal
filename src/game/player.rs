use strum_macros::EnumCount;

use super::deck::Card;
pub const NUMBER_OF_PLAYERS: usize = 4;

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, EnumCount)]
pub enum Team {
    A,
    B,
}

impl Team {
    pub fn to_index(&self) -> usize {
        match self {
            Self::A => 0,
            Self::B => 1,
        }
    }

    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Self::A,
            1 => Self::B,
            _ => panic!("Invalid index for converting to Team")
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Hand {
    pub hand: Vec<Card>,
}

impl Hand {
    pub fn new(cards: Vec<Card>) -> Self {
        Self { hand: cards }
    }

    pub fn take_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    pub fn sort(&mut self) {
        self.hand
            .sort_by_key(|card| (card.suit.clone(), card.value.clone()));
    }

    pub fn cards(&self) -> &Vec<Card> {
        &self.hand
    }

    pub fn into_cards(self) -> Vec<Card> {
        self.hand
    }

    pub fn empty(&self) -> bool {
        self.cards().len() == 0
    }

    pub fn remove_card(&mut self, card: &Card) -> Option<Card> {
        let card_position = self.hand.iter().position(|hand_card| *hand_card == *card)?;
        let removed_card = self.hand.remove(card_position);

        Some(removed_card)
    }
}

#[derive(Default, Debug, Clone)]
pub struct Player {
    pub name: String,
    pub hand: Hand,
    pub index: usize,
}

impl Player {
    pub fn recieve_cards(&mut self, cards: Vec<Card>) {
        self.hand = Hand { hand: cards }
    }

    pub fn recieve_card(&mut self, card: Card) {
        self.hand.take_card(card);
    }

    pub fn sort_hand(&mut self) {
        self.hand.sort();
    }

    pub fn get_hand(&self) -> &Hand {
        &self.hand
    }

    pub fn get_team(&self) -> Team {
        if self.index % 2 == 0 {
            Team::A
        } else {
            Team::B
        }
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn has_cards(&self) -> bool {
        !self.hand.empty()
    }

    pub fn remove_card(&mut self, card: &Card) -> Option<Card> {
        self.hand.remove_card(card)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Players {
    player_turn: usize,
    pub players: [Player; NUMBER_OF_PLAYERS],
}

impl Players {
    pub fn new() -> Self {
        let mut players = Players::default();
        players.set_turn(0);
        for (index, player) in players.players.iter_mut().enumerate() {
            player.index = index;
        }

        players
    }

    pub fn give_card_to_next_player(&mut self, card: Card) {
        self.players[self.player_turn].recieve_card(card);
        self.increment_turn();
    }

    pub fn set_turn(&mut self, turn: usize) {
        assert!(turn < NUMBER_OF_PLAYERS);
        self.player_turn = turn;
    }

    pub fn increment_turn(&mut self) {
        self.player_turn += 1;
        self.player_turn %= NUMBER_OF_PLAYERS;
    }

    pub fn sort_hands(&mut self) {
        for player in self.players.as_mut() {
            player.sort_hand();
        }
    }

    pub fn get(&self, index: usize) -> Option<&Player> {
        self.players.get(index)
    }

    pub fn have_cards(&self) -> bool {
        self.players.iter().any(|player| player.has_cards())
    }
}

impl<'a> IntoIterator for &'a Players {
    type Item = &'a Player;
    type IntoIter = std::slice::Iter<'a, Player>;

    fn into_iter(self) -> Self::IntoIter {
        self.players.iter()
    }
}

impl<'a> IntoIterator for &'a mut Players {
    type Item = &'a mut Player;
    type IntoIter = std::slice::IterMut<'a, Player>;

    fn into_iter(self) -> Self::IntoIter {
        self.players.iter_mut()
    }
}
