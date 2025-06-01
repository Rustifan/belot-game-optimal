use rand::random_range;
use strum::EnumCount;

use crate::game::deck::CardSuit;


pub fn get_random_suit()->CardSuit{
    let number = random_range(0..CardSuit::COUNT);
    match number {
        0 => CardSuit::Leaf,
        1 => CardSuit::Pumpkin, 
        2 => CardSuit::Herz,
        3 => CardSuit::Acorn,
        _ => CardSuit::Leaf, // Fallback to Leaf if out of range
    }
}
