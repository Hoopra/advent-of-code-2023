use crate::scratch_card::ScratchCard;
use std::{collections::HashMap, fs::read_to_string};

pub fn get_total_cards_won(file_path: &str) -> usize {
    let text = read_to_string(file_path).unwrap();

    let mut resolved_cards: Vec<u32> = Vec::new();
    let mut unresolved_cards: Vec<u32> = Vec::new();
    let mut winnings_by_id: HashMap<u32, u32> = HashMap::new();

    text.lines().for_each(|line| {
        let scratch_card = ScratchCard::new(line);
        let id = scratch_card.id;
        let score = scratch_card.find_scoring_numbers().len();

        winnings_by_id.insert(id, score as u32);
        unresolved_cards.push(id)
    });

    resolve_cards(&mut unresolved_cards, &mut resolved_cards, &winnings_by_id);

    resolved_cards.len()
}

fn resolve_cards(
    unresolved_cards: &mut Vec<u32>,
    resolved_cards: &mut Vec<u32>,
    winnings_by_id: &HashMap<u32, u32>,
) {
    while unresolved_cards.len() > 0 {
        let next = unresolved_cards.pop().unwrap_or(0);
        if next == 0 {
            break;
        }

        resolved_cards.push(next);

        let winnings = *winnings_by_id.get(&next).unwrap_or(&0);
        if winnings == 0 {
            continue;
        }

        let mut won_cards = get_won_cards(next, winnings);
        unresolved_cards.append(&mut won_cards);
    }
}

fn get_won_cards(id: u32, winnings: u32) -> Vec<u32> {
    let mut result = vec![];
    let mut add = 1;

    while add <= winnings {
        result.push(id + add);
        add += 1;
    }

    result
}
