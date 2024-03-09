use std::fs::read_to_string;

use crate::scratch_card::ScratchCard;

pub fn get_all_card_scores(file_path: &str) -> u32 {
    let text = read_to_string(file_path).unwrap();

    text.lines()
        .map(|line| {
            let scratch_card = ScratchCard::new(line);
            scratch_card.find_score()
        })
        .sum::<u32>()
}
