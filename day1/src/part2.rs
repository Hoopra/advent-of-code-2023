use std::{collections::HashMap, fs::read_to_string};

type DigitResult = (usize, char);

pub fn solve_part_2(file_path: &str) {
    let file = read_to_string(file_path).unwrap();

    let sum = file.lines().fold(0, |previous, next| {
        let number = find_calibration_number(next);

        previous + number
    });

    assert_eq!(sum, 54277);
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
    let (numerical_index, numerical_value) = find_numerical_digit_left(input);
    let (spelled_index, spelled_value) = find_spelled_digit_left(input);

    match numerical_index <= spelled_index {
        true => numerical_value,
        false => spelled_value,
    }
}

fn find_digit_right(input: &str) -> char {
    let (numerical_index, numerical_value) = find_numerical_digit_right(input);
    let (spelled_index, spelled_value) = find_spelled_digit_right(input);

    match numerical_index <= spelled_index {
        true => numerical_value,
        false => spelled_value,
    }
}

fn find_numerical_digit_left(input: &str) -> DigitResult {
    let result = input
        .chars()
        .enumerate()
        .find(|(_, character)| character.is_ascii_digit());

    let result = result.unwrap_or((input.len(), '\0'));
    (result.0, result.1)
}

fn find_numerical_digit_right(input: &str) -> DigitResult {
    let as_vec: Vec<char> = input.chars().collect();

    let result = as_vec
        .iter()
        .enumerate()
        .rfind(|(_, character)| character.is_ascii_digit());

    let result = result.unwrap_or((0, &'\0'));
    (input.len() - result.0, result.1.to_owned())
}

fn get_spelled_digits() -> HashMap<&'static str, char> {
    HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ])
}

fn find_spelled_digit_left(input: &str) -> DigitResult {
    let spelled_digits = get_spelled_digits();

    let mut result = (input.len(), '\0');

    let character_string: String = input.to_owned();

    for (spelling, character) in spelled_digits.iter() {
        let found = character_string.find(spelling);

        if found.is_none() {
            continue;
        }

        let found_index = found.unwrap();

        if found_index < result.0 {
            result = (found_index, character.to_owned());
        }
    }

    result
}

fn find_spelled_digit_right(input: &str) -> DigitResult {
    let spelled_digits = get_spelled_digits();

    let length = input.len();
    let mut result = (length, '\0');

    let character_string: String = input.to_owned();

    for (spelling, character) in spelled_digits.iter() {
        let found = character_string.rfind(spelling);

        if found.is_none() {
            continue;
        }

        let found_index = length - found.unwrap();

        if found_index < result.0 {
            result = (found_index, character.to_owned());
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
