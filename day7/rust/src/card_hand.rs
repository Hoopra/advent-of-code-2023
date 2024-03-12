use std::{
    cmp::Ordering,
    collections::HashMap,
    ops::{Deref, Sub},
};

type Card = u16;

#[derive(PartialEq, Debug, PartialOrd, Copy, Clone, Eq, Ord)]
enum HandStrength {
    None = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfKind = 4,
    FullHouse = 5,
    FourOfKind = 6,
    FiveOfKind = 7,
}

impl Sub<HandStrength> for HandStrength {
    type Output = u16;

    fn sub(self, rhs: HandStrength) -> u16 {
        (self as u16) - (rhs as u16)
    }
}

impl Deref for HandStrength {
    type Target = HandStrength;

    fn deref(&self) -> &Self::Target {
        &self
    }
}

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

fn hand_strength_from_cards(cards: &Vec<Card>) -> HandStrength {
    let mut map: HashMap<u16, u16> = HashMap::new();

    for card in cards.iter() {
        let current = map.get(card).unwrap_or(&0);
        map.insert(*card, current + 1);
    }

    let values: Vec<u16> = map.values().map(|value| *value).collect();

    if values.contains(&5) {
        return HandStrength::FiveOfKind;
    }

    if values.contains(&4) {
        return HandStrength::FourOfKind;
    }

    if values.contains(&3) {
        return match values.contains(&2) {
            true => HandStrength::FullHouse,
            false => HandStrength::ThreeOfKind,
        };
    }

    let pairs = values.iter().filter(|value| *value == &2).count();

    match pairs {
        2 => HandStrength::TwoPair,
        1 => HandStrength::OnePair,
        _ => HandStrength::None,
    }
}

#[cfg(test)]
mod test_toy_boat {
    use super::*;

    #[test]
    fn evaluates_hand_strength() {
        assert_eq!(
            hand_strength_from_cards(&vec![14, 14, 14, 14, 14]),
            HandStrength::FiveOfKind
        );

        assert_eq!(
            hand_strength_from_cards(&vec![14, 14, 14, 14, 13]),
            HandStrength::FourOfKind
        );

        assert_eq!(
            hand_strength_from_cards(&vec![14, 14, 14, 13, 13]),
            HandStrength::FullHouse
        );

        assert_eq!(
            hand_strength_from_cards(&vec![14, 14, 14, 13, 12]),
            HandStrength::ThreeOfKind
        );

        assert_eq!(
            hand_strength_from_cards(&vec![14, 14, 13, 13, 12]),
            HandStrength::TwoPair
        );

        assert_eq!(
            hand_strength_from_cards(&vec![14, 14, 13, 12, 11]),
            HandStrength::OnePair
        );

        assert_eq!(
            hand_strength_from_cards(&vec![14, 13, 12, 11, 10]),
            HandStrength::None
        );
    }
}
