#![allow(dead_code)]

use std::fs::read_to_string;

pub fn solve_day_1_part_1(file_path: &str) {
    let file = read_to_string(file_path).unwrap();

    let mut sum = 0;
    let mut lines = 0;

    for line in file.split_whitespace() {
        let first_digit = find_digit_from_start(line);
        let last_digit = find_digit_from_end(line);

        let number = concat_chars_to_number(first_digit, last_digit);
        let number = number.parse::<u32>().expect("invalid number");

        sum += number;
        lines += 1;
    }

    assert_eq!(lines, 1000);
    println!("advent 1 grand total: {sum}");
}

fn concat_chars_to_number(first: char, last: char) -> String {
    let mut number = String::from(first);
    number.push(last);

    number
}

fn find_digit_from_start<'a>(input: &'a str) -> char {
    find_digit(input.chars())
}

fn find_digit_from_end<'a>(input: &'a str) -> char {
    find_digit(input.chars().rev())
}

fn find_digit(chars: impl Iterator<Item = char>) -> char {
    for character in chars {
        if character.is_digit(10) {
            return character;
        }
    }

    '\0'
}
