use super::deck::Card;

#[derive(Debug)]
pub struct Trick {
    player_index_turn: usize,
    cards_on_table: Vec<Card>,
}

impl Trick {
    pub fn new(player_index_turn: usize)->Self{
        Trick {
            cards_on_table: vec![],
            player_index_turn
        }
    }

    pub fn cards(&self) -> &Vec<Card> {
        &self.cards_on_table
    }

    pub fn play_card(&mut self, card: Card) {
        self.cards_on_table.push(card);
        self.player_index_turn += 1;
    }

    pub fn get_player_index_turn(&self) -> usize {
        self.player_index_turn
    }

    pub fn into_cards(self) -> Vec<Card> {
        self.cards_on_table
    }
}
