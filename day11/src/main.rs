mod model;

use model::StarMap;
use std::fs::read_to_string;

fn main() {
    let file_path = "input.txt";
    let text = read_to_string(file_path).unwrap();

    solve_part_1(&text);
    solve_part_2(&text);
}

fn solve_part_1(text: &str) {
    let mut star_map = StarMap::new(&text);
    star_map.expand();

    let result = star_map.sum_galaxy_pair_distances();

    assert_eq!(result, 9565386);
}

fn solve_part_2(text: &str) {
    let mut star_map = StarMap::new(&text);
    star_map.expand_by_factor(1_000_000);

    let result = star_map.sum_galaxy_pair_distances();

    assert_eq!(result, 857986849428);
}
