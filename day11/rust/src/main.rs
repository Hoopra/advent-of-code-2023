mod model;

use model::{Position, StarMap};
use std::fs::read_to_string;

fn main() {
    let file_path = "../input.txt";
    let text = read_to_string(file_path).unwrap();

    let mut star_map = StarMap::new(&text);
    star_map.expand();

    let pairs = star_map.find_galaxy_pairs();

    let result = pairs.iter().fold(0, |sum, (pair_a, pair_b)| {
        sum + distance_between_positions(pair_a, pair_b)
    });

    assert_eq!(result, 9565386);
}

fn distance_between_positions(a: &Position, b: &Position) -> usize {
    let (xa, ya) = a;
    let (xb, yb) = b;

    let x = steps_between_coordinates(xa, xb);
    let y = steps_between_coordinates(ya, yb);

    x + y
}

fn steps_between_coordinates(a: &usize, b: &usize) -> usize {
    match a > b {
        true => a - b,
        _ => b - a,
    }
}
