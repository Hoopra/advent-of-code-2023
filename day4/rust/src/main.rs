use std::fs::read_to_string;

struct ScratchCard {
    numbers: Vec<u32>,
    winning_numbers: Vec<u32>,
}

impl ScratchCard {
    pub fn new(text: &str) -> ScratchCard {
        let (_, numbers_part) = text.split_at(8);

        let parts: Vec<&str> = numbers_part.split('|').collect();
        let part1 = parts.get(0);
        let part2 = parts.get(1);

        ScratchCard {
            winning_numbers: find_numbers_in_text(part1.unwrap_or(&"")),
            numbers: find_numbers_in_text(part2.unwrap_or(&"")),
        }
    }
}

impl ScratchCard {
    pub fn find_score(&self) -> u32 {
        let scoring_numbers = self.find_scoring_numbers();
        println!("scoring numbers {}", scoring_numbers.len());

        calculate_score_for_numbers(&scoring_numbers)
    }

    pub fn find_scoring_numbers(&self) -> Vec<u32> {
        self.numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .map(|number| *number)
            .collect()
    }
}

fn calculate_score_for_numbers(scoring_numbers: &Vec<u32>) -> u32 {
    scoring_numbers.iter().fold(0, |score, _| {
        if score == 0 {
            return 1;
        }
        score * 2
    })
}

fn find_numbers_in_text(text: &str) -> Vec<u32> {
    text.split_whitespace()
        .map(|value| value.parse::<u32>().unwrap_or(0))
        .collect()
}

fn main() {
    let file_path = "../input.txt";
    let text = read_to_string(file_path).unwrap();
    let scores = text
        .lines()
        .map(|line| {
            let scratch_card = ScratchCard::new(line);
            let score = scratch_card.find_score();

            println!("card score {score}");

            score
        })
        .sum::<u32>();

    println!("grand total: {scores}");
}
