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

enum Direction {
    Forward,
    Backward,
}

pub fn solve_day_1_part_2(file_path: &str) {
    let file = read_to_string(file_path).unwrap();

    let mut sum = 0;
    let mut lines = 0;

    for line in file.split_whitespace() {
        let (first_digit, last_digit) = find_digits(line);

        let number = concat_chars_to_number(first_digit, last_digit);

        println!("{line}: {number} ({first_digit}, {last_digit})");

        sum += number;
        lines += 1;

        if lines >= 25 {
            break;
        }
    }

    // assert_eq!(lines, 1000);
    println!("advent 2 grand total: {sum}");
}

fn concat_chars_to_number(first: char, last: char) -> u32 {
    let mut number = String::from(first);
    number.push(last);

    number.parse::<u32>().expect("invalid number")
}

fn find_digits(input: &str) -> (char, char) {
    let first_digit = find_digit_from_start(input);
    let last_digit = find_digit_from_end(input);

    (first_digit, last_digit)
}

fn find_digit_from_start<'a>(input: &'a str) -> char {
    let chars: Vec<char> = input.chars().collect();

    let numerical = find_numerical_digit(&chars, Direction::Forward);
    let spelled = find_spelled_digit(&chars, Direction::Forward);

    if numerical.index < spelled.index {
        return numerical.value;
    }

    spelled.value
}

fn find_digit_from_end<'a>(input: &'a str) -> char {
    let chars: Vec<char> = input.chars().collect();
    let chars_reversed: Vec<char> = input.chars().rev().collect();

    let numerical = find_numerical_digit(&chars_reversed, Direction::Backward);
    let spelled = find_spelled_digit(&chars, Direction::Backward);

    if numerical.index >= spelled.index {
        return numerical.value;
    }

    spelled.value
}

fn find_numerical_digit(chars: &Vec<char>, direction: Direction) -> DigitResult {
    for (index, character) in chars.iter().enumerate() {
        if character.is_digit(10) {
            return DigitResult::new(character.to_owned(), index);
        }
    }

    DigitResult::new('\0', get_start_index(&direction, chars.len()))
}

fn find_spelled_digit(chars: &Vec<char>, direction: Direction) -> DigitResult {
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

    let start_index = get_start_index(&direction, chars.len());

    let mut result = DigitResult::new('\0', start_index);

    let character_string: String = chars.into_iter().collect();

    for (spelling, character) in spelled_digits.iter() {
        let found = character_string(spelling);

        if found.is_none() {
            continue;
        }

        let found_index = found.unwrap();

        match direction {
            Direction::Backward => {
                if found_index > result.index {
                    result = DigitResult::new(character.to_owned(), found_index);
                }
            }
            Direction::Forward => {
                if found_index < result.index {
                    result = DigitResult::new(character.to_owned(), found_index);
                }
            }
        }
    }

    result
}

fn get_start_index(direction: &Direction, size: usize) -> usize {
    match direction {
        Direction::Backward => 0,
        Direction::Forward => size,
    }
}

#[cfg(test)]
mod test_find_digit {
    use super::*;

    #[test]
    fn find_first_digit() {
        assert_digits("19qdlpmdrxone7sevennine", '1', '9');

        assert_digits("joneseven2sseven64chvczzn", '1', '4');

        assert_digits("seven82683", '7', '3');

        assert_digits("two1nine", '2', '9');

        assert_digits("eightwothree", '8', '3');

        assert_digits("abcone2threexyz", '1', '3');

        assert_digits("xtwone3four", '2', '4');

        assert_digits("4nineeightseven2", '4', '2');

        assert_digits("zoneight234", '1', '4');

        assert_digits("7pqrstsixteen", '7', '6');
    }

    fn assert_digits(input: &str, digit_1: char, digit_2: char) {
        let (first_digit, last_digit) = find_digits(input);

        assert_eq!(first_digit, digit_1);
        assert_eq!(last_digit, digit_2);
    }
}
