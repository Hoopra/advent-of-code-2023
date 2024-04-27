use crate::{
    card_hand::CardHand,
    hand_strength::{hand_strength_from_card_values, values_from_cards, Card, HandStrength},
};

pub fn use_jokers_in_card_hands(card_hands: Vec<CardHand>, joker_value: Card) -> Vec<CardHand> {
    card_hands
        .iter()
        .map(|card_hand| replace_jokers(card_hand, joker_value))
        .collect()
}

fn replace_jokers(card_hand: &CardHand, joker_value: Card) -> CardHand {
    let cards = card_hand.get_cards();

    let new_cards: Vec<Card> = cards
        .into_iter()
        .map(|card| {
            let card = *card;
            if card == joker_value {
                return 1;
            }

            card
        })
        .collect();

    let strength = augment_hand_strength_with_jokers(&card_hand.get_cards(), joker_value);

    card_hand.replace_cards(new_cards, strength)
}

fn augment_hand_strength_with_jokers(new_cards: &Vec<Card>, joker_value: Card) -> HandStrength {
    let non_jokers: Vec<Card> = new_cards
        .iter()
        .filter(|value| **value != joker_value)
        .map(|value| *value)
        .collect();

    let num_jokers = new_cards.len() - non_jokers.len();

    if num_jokers >= 4 {
        return HandStrength::FiveOfKind;
    }

    let mut card_values = values_from_cards(&non_jokers);

    let greatest_count = card_values.iter().max().unwrap_or(&1);
    let index = card_values
        .iter()
        .position(|value| value == greatest_count)
        .unwrap_or(0);

    card_values[index] = card_values[index] + (num_jokers as u16);

    hand_strength_from_card_values(card_values)
}
