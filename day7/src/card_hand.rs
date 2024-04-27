use std::cmp::Ordering;

use crate::hand_strength::{hand_strength_from_cards, Card, HandStrength};

#[derive(Ord, Debug, Clone)]
pub struct CardHand {
    strength: HandStrength,
    cards: Vec<Card>,
    pub bid: u32,
}

impl CardHand {
    pub fn new(input: &str) -> CardHand {
        let (cards, bid) = parse_card_hand_from_text(input);

        CardHand {
            strength: hand_strength_from_cards(&cards),
            cards,
            bid,
        }
    }
}

impl CardHand {
    pub fn get_cards(&self) -> &Vec<Card> {
        &self.cards
    }

    pub fn replace_cards(&self, cards: Vec<Card>, strength: HandStrength) -> CardHand {
        CardHand {
            strength,
            cards,
            bid: self.bid,
        }
    }
}

impl PartialEq for CardHand {
    fn eq(&self, other: &Self) -> bool {
        self.strength == other.strength
    }
}

impl Eq for CardHand {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialOrd for CardHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.strength > other.strength {
            return Some(Ordering::Greater);
        }

        if self.strength < other.strength {
            return Some(Ordering::Less);
        }

        // let range = self.cards.len()).map(|value| value as u16).enumerate();

        for (index, value) in self.cards.iter().enumerate() {
            let other_value = other.cards.get(index).unwrap_or(&0);

            if value > other_value || value < other_value {
                return Some(value.cmp(&other_value));
            }
        }

        Some(Ordering::Equal)
    }
}

fn parse_card_hand_from_text(text: &str) -> (Vec<Card>, u32) {
    let components: Vec<&str> = text.split_whitespace().collect();
    let card_text = components.get(0).unwrap();
    let bid_text = components.get(1).unwrap();

    let cards = card_text.chars().map(parse_card_from_text).collect();
    let bid = bid_text.parse::<u32>().unwrap();

    (cards, bid)
}

fn parse_card_from_text(character: char) -> Card {
    if character.is_ascii_digit() {
        return character.to_string().parse::<u16>().unwrap();
    }

    match character {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => panic!("invalid card"),
    }
}
