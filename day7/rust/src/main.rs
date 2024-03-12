mod card_hand;

use std::fs::read_to_string;

use card_hand::CardHand;

fn main() {
    let file_path = "../input.txt";
    let text = read_to_string(file_path).unwrap();

    let card_hands = parse_card_hands(text);
    let result = score_card_hands(card_hands);

    assert_eq!(result, 255048101);
}

fn order_card_hands(card_hands: Vec<CardHand>) -> Vec<CardHand> {
    let mut ordered = card_hands.to_vec();
    ordered.sort();

    ordered
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

fn parse_card_hands(text: String) -> Vec<CardHand> {
    text.lines().map(|line| CardHand::new(line)).collect()
}

#[cfg(test)]
mod test_toy_boat {
    use super::*;

    #[test]
    fn correctly_scores_card_hand_list() {
        let input = String::from("32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483");
        let card_hands = parse_card_hands(input);

        assert_eq!(score_card_hands(card_hands), 6440);
    }
}
