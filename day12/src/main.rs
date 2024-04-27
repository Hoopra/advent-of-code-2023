mod model;

use std::fs::read_to_string;

use model::SpringSchematic;

fn main() {
    let file_path = "../input.txt";

    solve_part_1(file_path);
    solve_part_2(file_path);
}

fn solve_part_1(file_path: &str) {
    let text = read_to_string(file_path).unwrap();

    let schematics = text.lines().map(|line| SpringSchematic::from_text(line));

    let result: usize = schematics
        .map(|schematic| schematic.find_damaged_combinations())
        .sum();

    assert_eq!(result, 7694)
}

fn solve_part_2(file_path: &str) {
    let text = read_to_string(file_path).unwrap();

    let schematics = text
        .lines()
        .map(|line| SpringSchematic::from_expanded_text(line));

    let result: usize = schematics
        .map(|schematic| schematic.find_damaged_combinations())
        .sum();

    assert_eq!(result, 5071883216318)
}
