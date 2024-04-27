use std::fs::read_to_string;

use crate::toy_boat::ToyBoat;

pub fn parse_single_game() {
    let (time, record) = parse_race_input("../input.txt");

    let number_of_beats = find_possible_record_beats(time, record);

    assert_eq!(number_of_beats, 28973936)
}

fn parse_race_input(file_path: &str) -> (u64, u64) {
    let text = read_to_string(file_path).unwrap();

    let lines: Vec<&str> = text.lines().collect();

    (
        parse_number_from_line(lines.get(0).unwrap()),
        parse_number_from_line(lines.get(1).unwrap()),
    )
}

fn parse_number_from_line(line: &str) -> u64 {
    line.split_whitespace()
        .filter(|value| value.chars().all(|character| character.is_ascii_digit()))
        .fold(String::from(""), |previous, next| previous + next)
        .parse()
        .unwrap()
}

fn find_possible_record_beats(time: u64, record: u64) -> u64 {
    let boat = ToyBoat::new(None);

    boat.calculate_distances_traveled(time)
        .iter()
        .filter(|value| value > &&record)
        .collect::<Vec<&u64>>()
        .len() as u64
}

#[cfg(test)]
mod test_toy_boat {
    use super::*;

    #[test]
    fn calculates_number_of_possible_record_beats() {
        assert_eq!(find_possible_record_beats(7, 9), 4);

        assert_eq!(find_possible_record_beats(15, 40), 8);
    }
}
