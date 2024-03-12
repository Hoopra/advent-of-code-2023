use std::{
    collections::HashMap,
    ops::{Deref, Sub},
};

pub type Card = u16;

#[derive(PartialEq, Debug, PartialOrd, Copy, Clone, Eq, Ord)]
pub enum HandStrength {
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

pub fn hand_strength_from_cards(cards: &Vec<Card>) -> HandStrength {
    let values = values_from_cards(cards);

    hand_strength_from_card_values(values)
}

pub fn values_from_cards(cards: &Vec<Card>) -> Vec<u16> {
    let mut map: HashMap<u16, u16> = HashMap::new();

    for card in cards.iter() {
        let current = map.get(card).unwrap_or(&0);
        map.insert(*card, current + 1);
    }

    map.values().map(|value| *value).collect()
}

pub fn hand_strength_from_card_values(values: Vec<u16>) -> HandStrength {
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
mod test_hand_strength {
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
