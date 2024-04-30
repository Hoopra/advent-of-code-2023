mod card_hand;
mod card_hand_with_jokers;
mod hand_strength;

use std::fs::read_to_string;

use card_hand::CardHand;
use card_hand_with_jokers::use_jokers_in_card_hands;

fn main() {
    let file_path = "input.txt";
    let text = read_to_string(file_path).unwrap();

    score_cards_from_text_no_jokers(&text);
    score_cards_from_text_with_jokers(&text);
}

fn order_card_hands(card_hands: Vec<CardHand>) -> Vec<CardHand> {
    let mut ordered = card_hands.to_vec();
    ordered.sort();

    ordered
}

fn score_cards_from_text_no_jokers(text: &String) {
    let card_hands = parse_card_hands(text);
    let result = score_card_hands(card_hands);

    assert_eq!(result, 255048101);
}

fn score_cards_from_text_with_jokers(text: &String) {
    let card_hands = parse_card_hands(text);
    let card_hands_with_jokers = use_jokers_in_card_hands(card_hands, 11);

    let result = score_card_hands(card_hands_with_jokers);

    assert_eq!(result, 253718286);
}

fn score_card_hands(card_hands: Vec<CardHand>) -> u32 {
    let ordered = order_card_hands(card_hands);

    ordered
        .iter()
        .enumerate()
        .fold(0, |previous, (index, next)| {
            previous + (index as u32 + 1) * next.bid
        })
}

fn parse_card_hands(text: &String) -> Vec<CardHand> {
    text.lines().map(|line| CardHand::new(line)).collect()
}

#[cfg(test)]
mod test_card_hand_scoring {
    use super::*;

    #[test]
    fn correctly_scores_card_hand_list_without_jokers() {
        let input = String::from("32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483");
        let card_hands = parse_card_hands(&input);

        assert_eq!(score_card_hands(card_hands), 6440);
    }

    #[test]
    fn correctly_scores_card_hand_list_with_jokers() {
        let input = String::from("32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483");
        let card_hands = parse_card_hands(&input);
        let with_jokers_applied = use_jokers_in_card_hands(card_hands, 11);

        assert_eq!(score_card_hands(with_jokers_applied), 5905);
    }
}
