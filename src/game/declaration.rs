use strum::IntoEnumIterator;

use super::{
    deck::{Card, CardSuit, CardValue},
    player::Hand,
};

#[derive(Debug, PartialEq, Eq)]
struct Declaration {
    points: usize,
    cards: Vec<Card>,
}

fn get_scale_index(card_value: &CardValue) -> usize {
    match card_value {
        CardValue::VII => 0,
        CardValue::VIII => 1,
        CardValue::IX => 2,
        CardValue::X => 3,
        CardValue::Jack => 4,
        CardValue::Queen => 5,
        CardValue::King => 6,
        CardValue::Kec => 7,
    }
}
fn get_scale_points(scale_len: usize) -> usize {
    match scale_len {
        3 => 20,
        4 => 50,
        5..=7 => 100,
        8 => 1000,
        _ => 0,
    }
}

fn get_points_from_four_of_a_kind(card_value: &CardValue) -> usize {
    match card_value {
        CardValue::VII => 0,
        CardValue::VIII => 0,
        CardValue::IX => 150,
        CardValue::X => 100,
        CardValue::Jack => 200,
        CardValue::Queen => 100,
        CardValue::King => 100,
        CardValue::Kec => 100,
    }
}

fn get_scales_by_suit(suit: &CardSuit, hand: &Hand) -> Vec<Declaration> {
    let mut result_declarations: Vec<Declaration> = vec![];
    let mut cards = hand
        .cards()
        .into_iter()
        .filter(|card| card.suit == *suit)
        .collect::<Vec<_>>();
    cards.sort_by_key(|card| get_scale_index(&card.value));
    println!("{cards:?}");
    let mut i = 0;
    while i < cards.len() {
        let mut card_val = get_scale_index(&cards[i].value);
        let mut j = i + 1;
        while j < cards.len() {
            let j_card = get_scale_index(&cards[j].value);
            let j_card_should_be = card_val + 1;
            if j_card != j_card_should_be {
                break;
            }
            card_val = get_scale_index(&cards[j].value);
            j += 1;
        }
        let in_row = j - i;
        let points = get_scale_points(in_row);
        if points == 0 {
            i = j;
            continue;
        }

        let declaration_cards = cards[i..j]
            .to_vec()
            .into_iter()
            .map(|card| card.clone())
            .collect::<Vec<_>>();
        let declaration = Declaration {
            points,
            cards: declaration_cards,
        };

        result_declarations.push(declaration);
        i = j;
    }

    result_declarations
}

fn get_scale_declarations(hand: &Hand) -> Vec<Declaration> {
    let mut result_declarations: Vec<Declaration> = vec![];
    for suit in CardSuit::iter() {
        let declarations = get_scales_by_suit(&suit, hand);
        result_declarations.extend(declarations);
    }
    result_declarations
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::deck::{Card, CardSuit, CardValue};
    use crate::game::player::Hand;

    struct ScaleTest {
        hand_cards: Vec<Card>,
        expected: Vec<Declaration>,
    }
    #[test]
    fn test_get_scales_by_suit() {
        let tests = vec![
            ScaleTest {
                hand_cards: vec![
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::VIII,
                    },
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::VII,
                    },
                    Card {
                        suit: CardSuit::Acorn,
                        value: CardValue::IX,
                    },
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::IX,
                    },
                ],
                expected: vec![Declaration {
                    points: 20,
                    cards: vec![
                        Card {
                            suit: CardSuit::Herz,
                            value: CardValue::VII,
                        },
                        Card {
                            suit: CardSuit::Herz,
                            value: CardValue::VIII,
                        },
                        Card {
                            suit: CardSuit::Herz,
                            value: CardValue::IX,
                        },
                    ],
                }],
            },
            ScaleTest {
                hand_cards: vec![
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::VIII,
                    },
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::VII,
                    },
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::IX,
                    },
                    Card {
                        suit: CardSuit::Acorn,
                        value: CardValue::VII,
                    },
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::X,
                    },
                ],
                expected: vec![Declaration {
                    points: 50,
                    cards: vec![
                        Card {
                            suit: CardSuit::Herz,
                            value: CardValue::VII,
                        },
                        Card {
                            suit: CardSuit::Herz,
                            value: CardValue::VIII,
                        },
                        Card {
                            suit: CardSuit::Herz,
                            value: CardValue::IX,
                        },
                        Card {
                            suit: CardSuit::Herz,
                            value: CardValue::X,
                        },
                    ],
                }],
            },
            ScaleTest {
                hand_cards: vec![
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::VIII,
                    },
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::VII,
                    },
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::IX,
                    },
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::X,
                    },
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::Jack,
                    },
                ],
                expected: vec![Declaration {
                    points: 100,
                    cards: vec![
                        Card {
                            suit: CardSuit::Herz,
                            value: CardValue::VII,
                        },
                        Card {
                            suit: CardSuit::Herz,
                            value: CardValue::VIII,
                        },
                        Card {
                            suit: CardSuit::Herz,
                            value: CardValue::IX,
                        },
                        Card {
                            suit: CardSuit::Herz,
                            value: CardValue::X,
                        },
                        Card {
                            suit: CardSuit::Herz,
                            value: CardValue::Jack,
                        },
                    ],
                }],
            },
            ScaleTest {
                hand_cards: vec![
                    Card {
                        suit: CardSuit::Leaf,
                        value: CardValue::VIII,
                    },
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::VII,
                    },
                    Card {
                        suit: CardSuit::Leaf,
                        value: CardValue::IX,
                    },
                    Card {
                        suit: CardSuit::Leaf,
                        value: CardValue::X,
                    },
                    Card {
                        suit: CardSuit::Leaf,
                        value: CardValue::Queen,
                    },
                    Card {
                        suit: CardSuit::Leaf,
                        value: CardValue::King,
                    },
                    Card {
                        suit: CardSuit::Acorn,
                        value: CardValue::Jack,
                    },
                    Card {
                        suit: CardSuit::Leaf,
                        value: CardValue::Kec,
                    },
                ],
                expected: vec![
                    Declaration {
                        points: 20,
                        cards: vec![
                            Card {
                                suit: CardSuit::Leaf,
                                value: CardValue::VIII,
                            },
                            Card {
                                suit: CardSuit::Leaf,
                                value: CardValue::IX,
                            },
                            Card {
                                suit: CardSuit::Leaf,
                                value: CardValue::X,
                            },
                        ],
                    },
                    Declaration {
                        points: 20,
                        cards: vec![
                            Card {
                                suit: CardSuit::Leaf,
                                value: CardValue::Queen,
                            },
                            Card {
                                suit: CardSuit::Leaf,
                                value: CardValue::King,
                            },
                            Card {
                                suit: CardSuit::Leaf,
                                value: CardValue::Kec,
                            },
                        ],
                    },
                ],
            },
            ScaleTest {
                hand_cards: vec![
                    Card {
                        suit: CardSuit::Pumpkin,
                        value: CardValue::VIII,
                    },
                    Card {
                        suit: CardSuit::Pumpkin,
                        value: CardValue::VII,
                    },
                    Card {
                        suit: CardSuit::Pumpkin,
                        value: CardValue::IX,
                    },
                    Card {
                        suit: CardSuit::Pumpkin,
                        value: CardValue::X,
                    },
                    Card {
                        suit: CardSuit::Acorn,
                        value: CardValue::Jack,
                    },
                    Card {
                        suit: CardSuit::Pumpkin,
                        value: CardValue::Queen,
                    },
                    Card {
                        suit: CardSuit::Pumpkin,
                        value: CardValue::Jack,
                    },
                ],
                expected: vec![Declaration {
                    points: 100,
                    cards: vec![
                        Card {
                            suit: CardSuit::Pumpkin,
                            value: CardValue::VII,
                        },
                        Card {
                            suit: CardSuit::Pumpkin,
                            value: CardValue::VIII,
                        },
                        Card {
                            suit: CardSuit::Pumpkin,
                            value: CardValue::IX,
                        },
                        Card {
                            suit: CardSuit::Pumpkin,
                            value: CardValue::X,
                        },
                        Card {
                            suit: CardSuit::Pumpkin,
                            value: CardValue::Jack,
                        },
                        Card {
                            suit: CardSuit::Pumpkin,
                            value: CardValue::Queen,
                        },
                    ],
                }],
            },
            ScaleTest {
                hand_cards: vec![
                    Card {
                        suit: CardSuit::Acorn,
                        value: CardValue::VIII,
                    },
                    Card {
                        suit: CardSuit::Acorn,
                        value: CardValue::VII,
                    },
                    Card {
                        suit: CardSuit::Acorn,
                        value: CardValue::IX,
                    },
                    Card {
                        suit: CardSuit::Acorn,
                        value: CardValue::X,
                    },
                    Card {
                        suit: CardSuit::Acorn,
                        value: CardValue::Jack,
                    },
                    Card {
                        suit: CardSuit::Acorn,
                        value: CardValue::Queen,
                    },
                    Card {
                        suit: CardSuit::Acorn,
                        value: CardValue::Kec,
                    },
                    Card {
                        suit: CardSuit::Acorn,
                        value: CardValue::King,
                    },
                ],
                expected: vec![Declaration {
                    points: 1000,
                    cards: vec![
                        Card {
                            suit: CardSuit::Acorn,
                            value: CardValue::VII,
                        },
                        Card {
                            suit: CardSuit::Acorn,
                            value: CardValue::VIII,
                        },
                        Card {
                            suit: CardSuit::Acorn,
                            value: CardValue::IX,
                        },
                        Card {
                            suit: CardSuit::Acorn,
                            value: CardValue::X,
                        },
                        Card {
                            suit: CardSuit::Acorn,
                            value: CardValue::Jack,
                        },
                        Card {
                            suit: CardSuit::Acorn,
                            value: CardValue::Queen,
                        },
                        Card {
                            suit: CardSuit::Acorn,
                            value: CardValue::King,
                        },
                        Card {
                            suit: CardSuit::Acorn,
                            value: CardValue::Kec,
                        },
                    ],
                }],
            },
            ScaleTest {
                hand_cards: vec![
                    Card {
                        suit: CardSuit::Leaf,
                        value: CardValue::X,
                    },
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::VII,
                    },
                    Card {
                        suit: CardSuit::Leaf,
                        value: CardValue::IX,
                    },
                    Card {
                        suit: CardSuit::Leaf,
                        value: CardValue::Queen,
                    },
                    Card {
                        suit: CardSuit::Acorn,
                        value: CardValue::Jack,
                    },
                    Card {
                        suit: CardSuit::Leaf,
                        value: CardValue::Kec,
                    },
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::King,
                    },
                    Card {
                        suit: CardSuit::Pumpkin,
                        value: CardValue::VIII,
                    },
                ],
                expected: vec![],
            },
            ScaleTest {
                hand_cards: vec![
                    Card {
                        suit: CardSuit::Pumpkin,
                        value: CardValue::VII,
                    },
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::VII,
                    },
                    Card {
                        suit: CardSuit::Leaf,
                        value: CardValue::X,
                    },
                    Card {
                        suit: CardSuit::Leaf,
                        value: CardValue::IX,
                    },
                    Card {
                        suit: CardSuit::Leaf,
                        value: CardValue::Queen,
                    },
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::IX,
                    },
                    Card {
                        suit: CardSuit::Leaf,
                        value: CardValue::Jack,
                    },
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::VIII,
                    },
                ],
                expected: vec![
                    Declaration {
                        points: 50,
                        cards: vec![
                            Card {
                                suit: CardSuit::Leaf,
                                value: CardValue::IX,
                            },
                            Card {
                                suit: CardSuit::Leaf,
                                value: CardValue::X,
                            },
                            Card {
                                suit: CardSuit::Leaf,
                                value: CardValue::Jack,
                            },
                            Card {
                                suit: CardSuit::Leaf,
                                value: CardValue::Queen,
                            },
                        ],
                    },
                    Declaration {
                        points: 20,
                        cards: vec![
                            Card {
                                suit: CardSuit::Herz,
                                value: CardValue::VII,
                            },
                            Card {
                                suit: CardSuit::Herz,
                                value: CardValue::VIII,
                            },
                            Card {
                                suit: CardSuit::Herz,
                                value: CardValue::IX,
                            },
                        ],
                    },
                ],
            },
        ];

        for test in tests {
            let hand = Hand::new(test.hand_cards);
            let result = get_scale_declarations(&hand);
            assert_eq!(result, test.expected);
        }
    }
}
