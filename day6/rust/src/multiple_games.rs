use std::fs::read_to_string;

use crate::toy_boat::ToyBoat;

pub fn parse_multiple_games() {
    let (times, records) = parse_race_input("../input.txt");
    let mut product = 1;

    for index in 0..times.len() {
        let time = times.get(index).unwrap();
        let record = records.get(index).unwrap();

        let number_of_beats = find_possible_record_beats(*time, *record);

        product *= number_of_beats;
    }

    assert_eq!(product, 4568778)
}

fn parse_race_input(file_path: &str) -> (Vec<u64>, Vec<u64>) {
    let text = read_to_string(file_path).unwrap();

    let lines: Vec<&str> = text.lines().collect();

    (
        parse_numbers_from_line(lines.get(0).unwrap()),
        parse_numbers_from_line(lines.get(1).unwrap()),
    )
}

fn parse_numbers_from_line(line: &str) -> Vec<u64> {
    line.split_whitespace()
        .filter(|value| value.chars().all(|character| character.is_ascii_digit()))
        .map(|value| value.parse().unwrap())
        .collect()
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
