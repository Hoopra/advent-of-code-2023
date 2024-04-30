mod model;
mod parse;
mod part1;
mod part2;

use part1::calculate_necessary_steps;
use part2::calculate_necessary_ghost_steps;
use std::fs::read_to_string;

fn main() {
    let file_path = "input.txt";
    let text = read_to_string(file_path).unwrap();

    let steps_taken = calculate_necessary_steps(&text);
    assert_eq!(steps_taken, 23147);

    let steps_taken = calculate_necessary_ghost_steps(&text);
    assert_eq!(steps_taken, 22289513667691);
}
