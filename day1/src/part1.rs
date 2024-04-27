#![allow(dead_code)]

use std::fs::read_to_string;

pub fn solve_part_1(file_path: &str) {
    let file = read_to_string(file_path).unwrap();

    let mut sum = 0;

    for line in file.lines() {
        let first_digit = find_digit_from_start(line);
        let last_digit = find_digit_from_end(line);

        let number = concat_chars_to_number(first_digit, last_digit);
        let number = number.parse::<u32>().expect("invalid number");

        sum += number;
    }

    assert_eq!(sum, 54390);
}

fn concat_chars_to_number(first: char, last: char) -> String {
    format!("{}{}", first, last)
}

fn find_digit_from_start(input: &str) -> char {
    find_digit(input.chars())
}

fn find_digit_from_end(input: &str) -> char {
    find_digit(input.chars().rev())
}

fn find_digit(chars: impl Iterator<Item = char>) -> char {
    for character in chars {
        if character.is_ascii_digit() {
            return character;
        }
    }

    '\0'
}
