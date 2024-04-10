mod energized;
mod model;

use crate::energized::find_energized_tiles;
use std::fs::read_to_string;

fn main() {
    solve_part_1("../input.txt");
}

fn solve_part_1(file_path: &str) {
    let text = read_to_string(file_path).unwrap();

    let energized = find_energized_tiles(&text, (0, 0));
    assert_eq!(energized.len(), 8249);
}
