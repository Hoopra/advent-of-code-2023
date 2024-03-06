use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug)]
struct DigitResult {
    value: char,
    index: usize,
}

impl DigitResult {
    fn new(value: char, index: usize) -> DigitResult {
        DigitResult { index, value }
    }
}

pub fn solve_day_1_part_2(file_path: &str) {
    let file = read_to_string(file_path).unwrap();

    let mut sum = 0;
    let mut lines = 0;

    for line in file.split_whitespace() {
        let number = find_calibration_number(line);

        sum += number;
        lines += 1;
    }

    assert_eq!(lines, 1000);
    println!("advent 2 grand total: {sum}"); // 54277
}

fn find_calibration_number(input: &str) -> u32 {
    let (first_digit, last_digit) = find_calibration_digits(input);

    concat_chars_to_number(first_digit, last_digit)
}

fn concat_chars_to_number(first: char, last: char) -> u32 {
    let mut number = String::from(first);
    number.push(last);

    number.parse::<u32>().expect("invalid number")
}

fn find_calibration_digits(input: &str) -> (char, char) {
    let first_digit = find_digit_left(input);
    let last_digit = find_digit_right(input);

    (first_digit, last_digit)
}

fn find_digit_left(input: &str) -> char {
    let chars: Vec<char> = input.chars().collect();

    let numerical = find_numerical_digit_left(&chars);
    let spelled = find_spelled_digit_left(&chars);

    if numerical.index <= spelled.index {
        return numerical.value;
    }

    spelled.value
}

fn find_digit_right(input: &str) -> char {
    let chars: Vec<char> = input.chars().collect();

    let numerical = find_numerical_digit_right(&chars);
    let spelled = find_spelled_digit_right(&chars);

    if numerical.index <= spelled.index {
        return numerical.value;
    }

    spelled.value
}

fn find_numerical_digit_left(chars: &Vec<char>) -> DigitResult {
    let result = chars
        .iter()
        .enumerate()
        .find(|(_, character)| character.is_ascii_digit());

    match result {
        Some((index, character)) => DigitResult::new(character.to_owned(), index),
        None => DigitResult::new('\0', chars.len()),
    }
}

fn find_numerical_digit_right(chars: &Vec<char>) -> DigitResult {
    let result = chars
        .iter()
        .enumerate()
        .rfind(|(_, character)| character.is_ascii_digit());

    let length = chars.len();

    match result {
        Some((index, character)) => DigitResult::new(character.to_owned(), length - index),
        None => DigitResult::new('\0', length),
    }
}

fn find_spelled_digit_left(chars: &Vec<char>) -> DigitResult {
    let spelled_digits: HashMap<&str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    let mut result = DigitResult::new('\0', chars.len());

    let character_string: String = chars.iter().collect();

    for (spelling, character) in spelled_digits.iter() {
        let found = character_string.find(spelling);

        if found.is_none() {
            continue;
        }

        let found_index = found.unwrap();

        if found_index < result.index {
            result = DigitResult::new(character.to_owned(), found_index);
        }
    }

    result
}

fn find_spelled_digit_right(chars: &Vec<char>) -> DigitResult {
    let spelled_digits: HashMap<&str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    let mut result = DigitResult::new('\0', chars.len());

    let character_string: String = chars.iter().collect();

    for (spelling, character) in spelled_digits.iter() {
        let found = character_string.rfind(spelling);

        if found.is_none() {
            continue;
        }

        let found_index = chars.len() - found.unwrap();

        if found_index < result.index {
            result = DigitResult::new(character.to_owned(), found_index);
        }
    }

    result
}

#[cfg(test)]
mod test_find_digit {
    use super::*;

    #[test]
    fn find_digits_left_works() {
        let input = "19qdlpmdrxone7sevennine";

        assert_eq!(find_digit_left(input), '1');
        assert_eq!(find_digit_right(input), '9');
    }

    #[test]
    fn find_calibration_number_works() {
        assert_eq!(find_calibration_number("19qdlpmdrxone7sevennine"), 19);

        assert_eq!(find_calibration_number("joneseven2sseven64chvczzn"), 14);

        assert_eq!(find_calibration_number("seven82683"), 73);

        assert_eq!(find_calibration_number("two1nine"), 29);

        assert_eq!(find_calibration_number("eightwothree"), 83);

        assert_eq!(find_calibration_number("abcone2threexyz"), 13);

        assert_eq!(find_calibration_number("xtwone3four"), 24);

        assert_eq!(find_calibration_number("4nineeightseven2"), 42);

        assert_eq!(find_calibration_number("zoneight234"), 14);

        assert_eq!(find_calibration_number("7pqrstsixteen"), 76);
    }
}
