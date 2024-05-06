use std::fs::read_to_string;

use crate::map::Map;

mod map;

fn main() {
    let text = read_to_string("input.txt").unwrap();

    solve_part_1(&text);
}

fn solve_part_1(text: &str) {
    let map = Map::from_text(&text);

    let result = map.reachable_positions(64);
    assert_eq!(result, 3666);
}
