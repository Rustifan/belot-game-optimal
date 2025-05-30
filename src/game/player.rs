use super::deck::Card;

const NUMBER_OF_PLAYERS: usize = 4;

#[derive(Default, Debug)]
pub struct Hand {
    hand: Vec<Card>,
}

impl Hand {
    pub fn take_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    pub fn sort(&mut self) {
        self.hand
            .sort_by_key(|card| (card.suit.clone(), card.value.clone()));
    }
}

#[derive(Default, Debug)]
pub struct Player {
    hand: Hand,
}

#[derive(Debug, Default)]
pub struct Players {
    player_turn: usize,
    pub players: [Player; NUMBER_OF_PLAYERS],
}

impl Players {
    pub fn new() -> Self {
        let mut players = Players::default();
        players.set_turn(0);

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
}

impl Player {
    pub fn recieve_card(&mut self, card: Card) {
        self.hand.take_card(card);
    }

    pub fn sort_hand(&mut self) {
        self.hand.sort();
    }
}
