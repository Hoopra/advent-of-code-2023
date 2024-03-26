mod model;
mod permutation;

use std::fs::read_to_string;

use model::SpringSchematic;

fn main() {
    let file_path = "../input.txt";
    let text = read_to_string(file_path).unwrap();

    let schematics = text.lines().map(|line| SpringSchematic::from_text(line));

    let result: usize = schematics
        .map(|schematic| schematic.find_damaged_combinations())
        .sum();

    assert_eq!(result, 7694)
}
