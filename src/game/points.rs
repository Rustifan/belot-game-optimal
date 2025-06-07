use super::deck::{Card, CardSuit, CardValue};

pub fn get_normal_points(card_value: &CardValue) -> usize {
    match card_value {
        CardValue::VII | CardValue::VIII | CardValue::IX => 0,
        CardValue::X => 10,
        CardValue::Jack => 2,
        CardValue::Queen => 3,
        CardValue::King => 4,
        CardValue::Kec => 11,
    }
}

pub fn get_trump_points(card_value: &CardValue) -> usize {
    match card_value {
        CardValue::VII | CardValue::VIII => 0,
        CardValue::X => 10,
        CardValue::Queen => 3,
        CardValue::King => 4,
        CardValue::Kec => 11,
        CardValue::IX => 14,
        CardValue::Jack => 20,
    }
}

pub fn better_than_normal(a: &Card, b: &Card) -> bool {
    let a_points = get_normal_points(&a.value);
    let b_points = get_normal_points(&b.value);

    if a_points == b_points {
        return a.value > b.value;
    }

    a_points > b_points
}

pub fn better_than_trump(a: &Card, b: &Card) -> bool {
    let a_points = get_trump_points(&a.value);
    let b_points = get_trump_points(&b.value);

    if a_points == b_points {
        return a.value > b.value;
    }

    a_points > b_points
}

pub fn get_best_normal(cards: &Vec<Card>) -> Option<Card> {
    let first_card = cards.get(0)?;
    let normal_suit = &first_card.suit;
    let best_card = cards
        .iter()
        .skip(1)
        .filter(|card| card.suit == *normal_suit)
        .fold(first_card, |acc, curr| {
            if better_than_normal(curr, acc) {
                curr
            } else {
                acc
            }
        });

    Some(best_card.clone())
}

pub fn get_best_trump(cards: &Vec<Card>, trump_color: CardSuit) -> Option<Card> {
    cards
        .iter()
        .filter(|card| card.suit == trump_color)
        .reduce(|a, b| if better_than_trump(a, b) { a } else { b })
        .map(|card| card.clone())
}
