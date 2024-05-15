mod map;
mod pathing;

use map::Map;
use std::fs::read_to_string;

use crate::pathing::find_longest_path_in_map;

fn main() {
    solve_part_1();
}

fn solve_part_1() {
    let text = read_to_string("input.txt").unwrap();
    let graph = Map::from_text(&text);

    let start = graph.find_start_position();
    let end = graph.find_end_position();

    let result = find_longest_path_in_map(&graph, &start, &end);
    assert_eq!(result, 2362);
}
