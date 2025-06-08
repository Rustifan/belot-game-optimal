use crate::game::{declaration::get_possible_declarations, player::Hand};

use super::{
    deck::{Card, CardSuit, CardValue},
    declaration::Declaration,
};

struct ScaleTest {
    hand_cards: Vec<Card>,
    expected: Vec<Declaration>,
}

#[test]
pub fn test_get_scales_by_suit() {
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
        ScaleTest {
            hand_cards: vec![
                Card {
                    suit: CardSuit::Acorn,
                    value: CardValue::X,
                },
                Card {
                    suit: CardSuit::Herz,
                    value: CardValue::X,
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
                    suit: CardSuit::Leaf,
                    value: CardValue::X,
                },
                Card {
                    suit: CardSuit::Pumpkin,
                    value: CardValue::X,
                },
                Card {
                    suit: CardSuit::Pumpkin,
                    value: CardValue::Kec,
                },
                Card {
                    suit: CardSuit::Pumpkin,
                    value: CardValue::King,
                },
            ],
            expected: vec![Declaration {
                points: 100,
                cards: vec![
                    Card {
                        suit: CardSuit::Leaf,
                        value: CardValue::X,
                    },
                    Card {
                        suit: CardSuit::Pumpkin,
                        value: CardValue::X,
                    },
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::X,
                    },
                    Card {
                        suit: CardSuit::Acorn,
                        value: CardValue::X,
                    },
                ],
            }],
        },
        ScaleTest {
            hand_cards: vec![
                Card {
                    suit: CardSuit::Acorn,
                    value: CardValue::IX,
                },
                Card {
                    suit: CardSuit::Herz,
                    value: CardValue::X,
                },
                Card {
                    suit: CardSuit::Herz,
                    value: CardValue::IX,
                },
                Card {
                    suit: CardSuit::Herz,
                    value: CardValue::Jack,
                },
                Card {
                    suit: CardSuit::Leaf,
                    value: CardValue::IX,
                },
                Card {
                    suit: CardSuit::Pumpkin,
                    value: CardValue::IX,
                },
                Card {
                    suit: CardSuit::Pumpkin,
                    value: CardValue::Kec,
                },
                Card {
                    suit: CardSuit::Pumpkin,
                    value: CardValue::King,
                },
            ],
            expected: vec![
                Declaration {
                    cards: vec![
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
                    points: 20,
                },
                Declaration {
                    points: 150,
                    cards: vec![
                        Card {
                            suit: CardSuit::Leaf,
                            value: CardValue::IX,
                        },
                        Card {
                            suit: CardSuit::Pumpkin,
                            value: CardValue::IX,
                        },
                        Card {
                            suit: CardSuit::Herz,
                            value: CardValue::IX,
                        },
                        Card {
                            suit: CardSuit::Acorn,
                            value: CardValue::IX,
                        },
                    ],
                },
            ],
        },
        ScaleTest {
            hand_cards: vec![
                Card {
                    suit: CardSuit::Acorn,
                    value: CardValue::Jack,
                },
                Card {
                    suit: CardSuit::Herz,
                    value: CardValue::VII,
                },
                Card {
                    suit: CardSuit::Herz,
                    value: CardValue::Jack,
                },
                Card {
                    suit: CardSuit::Leaf,
                    value: CardValue::VII,
                },
                Card {
                    suit: CardSuit::Leaf,
                    value: CardValue::Jack,
                },
                Card {
                    suit: CardSuit::Pumpkin,
                    value: CardValue::Jack,
                },
                Card {
                    suit: CardSuit::Pumpkin,
                    value: CardValue::VII,
                },
                Card {
                    suit: CardSuit::Acorn,
                    value: CardValue::VII,
                },
            ],
            expected: vec![Declaration {
                points: 200,
                cards: vec![
                    Card {
                        suit: CardSuit::Leaf,
                        value: CardValue::Jack,
                    },
                    Card {
                        suit: CardSuit::Pumpkin,
                        value: CardValue::Jack,
                    },
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::Jack,
                    },
                    Card {
                        suit: CardSuit::Acorn,
                        value: CardValue::Jack,
                    },
                ],
            }],
        },
        ScaleTest {
            hand_cards: vec![
                Card {
                    suit: CardSuit::Acorn,
                    value: CardValue::Jack,
                },
                Card {
                    suit: CardSuit::Herz,
                    value: CardValue::King,
                },
                Card {
                    suit: CardSuit::Herz,
                    value: CardValue::Jack,
                },
                Card {
                    suit: CardSuit::Leaf,
                    value: CardValue::King,
                },
                Card {
                    suit: CardSuit::Leaf,
                    value: CardValue::Jack,
                },
                Card {
                    suit: CardSuit::Pumpkin,
                    value: CardValue::Jack,
                },
                Card {
                    suit: CardSuit::Pumpkin,
                    value: CardValue::King,
                },
                Card {
                    suit: CardSuit::Acorn,
                    value: CardValue::King,
                },
            ],
            expected: vec![
                Declaration {
                    points: 100,
                    cards: vec![
                        Card {
                            suit: CardSuit::Leaf,
                            value: CardValue::King,
                        },
                        Card {
                            suit: CardSuit::Pumpkin,
                            value: CardValue::King,
                        },
                        Card {
                            suit: CardSuit::Herz,
                            value: CardValue::King,
                        },
                        Card {
                            suit: CardSuit::Acorn,
                            value: CardValue::King,
                        },
                    ],
                },
                Declaration {
                    points: 200,
                    cards: vec![
                        Card {
                            suit: CardSuit::Leaf,
                            value: CardValue::Jack,
                        },
                        Card {
                            suit: CardSuit::Pumpkin,
                            value: CardValue::Jack,
                        },
                        Card {
                            suit: CardSuit::Herz,
                            value: CardValue::Jack,
                        },
                        Card {
                            suit: CardSuit::Acorn,
                            value: CardValue::Jack,
                        },
                    ],
                },
            ],
        },
    ];

    for test in tests {
        let hand = Hand::new(test.hand_cards);
        let result = get_possible_declarations(&hand);
        assert_eq!(result, test.expected);
    }
}
