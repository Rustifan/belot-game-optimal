#[cfg(test)]
mod test_trick {
    use crate::game::player::{Hand, Player, Players};
    use crate::game::{deck::Card, trick::Trick};
    use crate::game::{deck::CardSuit, deck::CardValue};
    fn get_playeble_card_test_fn(
        cards_on_table: Vec<Card>,
        cards_in_hand: Vec<Card>,
        expacted_result: Vec<Card>,
        trump_color: CardSuit,
        player_index: usize,
    ) {
        let mut trick = Trick::new(player_index);
        trick.cards_on_table = cards_on_table;
        let mut players = Players::default();
        players.players[player_index] = Player {
            name: "test".to_string(),
            index: player_index,
            hand: Hand {
                hand: cards_in_hand,
            },
        };
        let result = trick.get_playeble_cards(&players, &trump_color);

        assert_eq!(result, expacted_result);
    }
    struct GetPlayebleCardsTest {
        cards_on_table: Vec<Card>,
        cards_in_hand: Vec<Card>,
        expacted: Vec<Card>,
        trump_color: CardSuit,
    }
    #[test]
    fn test_get_playeble_cards_empty_table() {
        let tests: Vec<GetPlayebleCardsTest> = vec![
            GetPlayebleCardsTest {
                cards_on_table: vec![Card {
                    suit: CardSuit::Acorn,
                    value: CardValue::Kec,
                }],
                cards_in_hand: vec![
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::VII,
                    },
                    Card {
                        suit: CardSuit::Acorn,
                        value: CardValue::VII,
                    },
                    Card {
                        suit: CardSuit::Pumpkin,
                        value: CardValue::Kec,
                    },
                    Card {
                        suit: CardSuit::Acorn,
                        value: CardValue::Jack,
                    },
                ],
                expacted: vec![
                    Card {
                        suit: CardSuit::Acorn,
                        value: CardValue::VII,
                    },
                    Card {
                        suit: CardSuit::Acorn,
                        value: CardValue::Jack,
                    },
                ],
                trump_color: CardSuit::Herz,
            },
            GetPlayebleCardsTest {
                cards_on_table: vec![Card {
                    suit: CardSuit::Leaf,
                    value: CardValue::IX,
                }],
                cards_in_hand: vec![
                    Card {
                        suit: CardSuit::Pumpkin,
                        value: CardValue::VII,
                    },
                    Card {
                        suit: CardSuit::Acorn,
                        value: CardValue::VII,
                    },
                    Card {
                        suit: CardSuit::Pumpkin,
                        value: CardValue::Kec,
                    },
                    Card {
                        suit: CardSuit::Acorn,
                        value: CardValue::Jack,
                    },
                ],
                expacted: vec![
                    Card {
                        suit: CardSuit::Pumpkin,
                        value: CardValue::VII,
                    },
                    Card {
                        suit: CardSuit::Acorn,
                        value: CardValue::VII,
                    },
                    Card {
                        suit: CardSuit::Pumpkin,
                        value: CardValue::Kec,
                    },
                    Card {
                        suit: CardSuit::Acorn,
                        value: CardValue::Jack,
                    },
                ],
                trump_color: CardSuit::Herz,
            },
            GetPlayebleCardsTest {
                cards_on_table: vec![
                    Card {
                        suit: CardSuit::Pumpkin,
                        value: CardValue::X,
                    },
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::Kec,
                    },
                ],
                cards_in_hand: vec![
                    Card {
                        suit: CardSuit::Pumpkin,
                        value: CardValue::VII,
                    },
                    Card {
                        suit: CardSuit::Pumpkin,
                        value: CardValue::Kec,
                    },
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::Jack,
                    },
                ],
                expacted: vec![
                    Card {
                        suit: CardSuit::Pumpkin,
                        value: CardValue::VII,
                    },
                    Card {
                        suit: CardSuit::Pumpkin,
                        value: CardValue::Kec,
                    },
                ],
                trump_color: CardSuit::Herz,
            },
            GetPlayebleCardsTest {
                cards_on_table: vec![
                    Card {
                        suit: CardSuit::Pumpkin,
                        value: CardValue::X,
                    },
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::X,
                    },
                    Card {
                        suit: CardSuit::Pumpkin,
                        value: CardValue::Kec,
                    },
                ],
                cards_in_hand: vec![
                    Card {
                        suit: CardSuit::Leaf,
                        value: CardValue::VII,
                    },
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::VII,
                    },
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::VIII,
                    },
                    Card {
                        suit: CardSuit::Acorn,
                        value: CardValue::Kec,
                    },
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::Jack,
                    },
                ],
                expacted: vec![Card {
                    suit: CardSuit::Herz,
                    value: CardValue::Jack,
                }],
                trump_color: CardSuit::Herz,
            },
            GetPlayebleCardsTest {
                cards_on_table: vec![
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
                        value: CardValue::Kec,
                    },
                ],
                cards_in_hand: vec![
                    Card {
                        suit: CardSuit::Leaf,
                        value: CardValue::VII,
                    },
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
                        value: CardValue::Kec,
                    },
                    Card {
                        suit: CardSuit::Acorn,
                        value: CardValue::Jack,
                    },
                ],
                expacted: vec![
                    Card {
                        suit: CardSuit::Leaf,
                        value: CardValue::VII,
                    },
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
                        value: CardValue::Kec,
                    },
                    Card {
                        suit: CardSuit::Acorn,
                        value: CardValue::Jack,
                    },
                ],
                trump_color: CardSuit::Herz,
            },
            GetPlayebleCardsTest {
                cards_on_table: vec![
                    Card {
                        suit: CardSuit::Pumpkin,
                        value: CardValue::X,
                    },
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::IX,
                    },
                    Card {
                        suit: CardSuit::Pumpkin,
                        value: CardValue::Kec,
                    },
                ],
                cards_in_hand: vec![
                    Card {
                        suit: CardSuit::Leaf,
                        value: CardValue::VII,
                    },
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
                        value: CardValue::Kec,
                    },
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::Jack,
                    },
                ],
                expacted: vec![Card {
                    suit: CardSuit::Herz,
                    value: CardValue::Jack,
                }],
                trump_color: CardSuit::Herz,
            },
            GetPlayebleCardsTest {
                cards_on_table: vec![
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::X,
                    },
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::IX,
                    },
                    Card {
                        suit: CardSuit::Pumpkin,
                        value: CardValue::Kec,
                    },
                ],
                cards_in_hand: vec![
                    Card {
                        suit: CardSuit::Leaf,
                        value: CardValue::VII,
                    },
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
                        value: CardValue::Kec,
                    },
                    Card {
                        suit: CardSuit::Herz,
                        value: CardValue::Jack,
                    },
                ],
                expacted: vec![Card {
                    suit: CardSuit::Herz,
                    value: CardValue::Jack,
                }],
                trump_color: CardSuit::Herz,
            },
            GetPlayebleCardsTest {
                cards_on_table: vec![Card {
                    suit: CardSuit::Herz,
                    value: CardValue::Jack,
                }],
                cards_in_hand: vec![
                    Card {
                        suit: CardSuit::Leaf,
                        value: CardValue::VII,
                    },
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
                        value: CardValue::Kec,
                    },
                ],
                expacted: vec![
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
                        value: CardValue::Kec,
                    },
                ],
                trump_color: CardSuit::Herz,
            },
        ];

        for test in tests {
            let cards_on_table_len = test.cards_on_table.len();
            get_playeble_card_test_fn(
                test.cards_on_table,
                test.cards_in_hand,
                test.expacted,
                test.trump_color,
                cards_on_table_len,
            );
        }
    }

    struct TrickWinnerTest {
        table: Vec<Card>,
        start_index: usize,
        trump: CardSuit,
        expected: Option<usize>,
    }

    #[test]
    fn test_get_trick_winner() {
        let test_cases = vec![
            TrickWinnerTest {
                table: vec![
                    Card {
                        value: CardValue::X,
                        suit: CardSuit::Pumpkin,
                    },
                    Card {
                        value: CardValue::VII,
                        suit: CardSuit::Pumpkin,
                    },
                    Card {
                        value: CardValue::Jack,
                        suit: CardSuit::Pumpkin,
                    },
                    Card {
                        value: CardValue::Kec,
                        suit: CardSuit::Acorn,
                    },
                ],
                start_index: 1,
                trump: CardSuit::Herz,
                expected: Some(1),
            },
            TrickWinnerTest {
                table: vec![
                    Card {
                        value: CardValue::Kec,
                        suit: CardSuit::Pumpkin,
                    },
                    Card {
                        value: CardValue::IX,
                        suit: CardSuit::Pumpkin,
                    },
                    Card {
                        value: CardValue::Jack,
                        suit: CardSuit::Pumpkin,
                    },
                    Card {
                        value: CardValue::Kec,
                        suit: CardSuit::Acorn,
                    },
                ],
                start_index: 2,
                trump: CardSuit::Herz,
                expected: Some(2),
            },
            TrickWinnerTest {
                table: vec![
                    Card {
                        value: CardValue::Kec,
                        suit: CardSuit::Pumpkin,
                    },
                    Card {
                        value: CardValue::Kec,
                        suit: CardSuit::Herz,
                    },
                    Card {
                        value: CardValue::Jack,
                        suit: CardSuit::Herz,
                    },
                    Card {
                        value: CardValue::IX,
                        suit: CardSuit::Pumpkin,
                    },
                ],
                start_index: 3,
                trump: CardSuit::Herz,
                expected: Some(1),
            },
            TrickWinnerTest {
                table: vec![
                    Card {
                        value: CardValue::Kec,
                        suit: CardSuit::Pumpkin,
                    },
                    Card {
                        value: CardValue::IX,
                        suit: CardSuit::Pumpkin,
                    },
                    Card {
                        value: CardValue::Jack,
                        suit: CardSuit::Pumpkin,
                    },
                    Card {
                        value: CardValue::Kec,
                        suit: CardSuit::Acorn,
                    },
                ],
                start_index: 2,
                trump: CardSuit::Herz,
                expected: Some(2),
            },
            TrickWinnerTest {
                table: vec![
                    Card {
                        value: CardValue::Kec,
                        suit: CardSuit::Pumpkin,
                    },
                    Card {
                        value: CardValue::IX,
                        suit: CardSuit::Pumpkin,
                    },
                    Card {
                        value: CardValue::Jack,
                        suit: CardSuit::Pumpkin,
                    },
                ],
                start_index: 2,
                trump: CardSuit::Herz,
                expected: None,
            },
        ];

        for test_case in test_cases {
            let mut trick = Trick::new(test_case.start_index);
            for card in test_case.table {
                trick.play_card(card);
            }
            let result = trick.get_trick_winner(&test_case.trump);
            assert_eq!(result, test_case.expected)
        }
    }
}
