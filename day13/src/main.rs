mod model;
mod util;

use model::TerrainMap;
use std::fs::read_to_string;

fn main() {
    let text = read_to_string("input.txt").unwrap();

    let map_texts: Vec<&str> = text.split("\n\n").collect();

    let maps: Vec<TerrainMap> = map_texts
        .iter()
        .map(|text| TerrainMap::from_text(text))
        .collect();

    solve_part_1(&maps);
    solve_part_2(&maps);
}

fn solve_part_1(maps: &Vec<TerrainMap>) {
    let result = maps.iter().fold(0, |sum, map| {
        let row = map.find_reflecting_row();
        let col = map.find_reflecting_col();

        sum + 100 * row.unwrap_or(0) + col.unwrap_or(0)
    });

    assert_eq!(result, 30535);
}

fn solve_part_2(maps: &Vec<TerrainMap>) {
    let result = maps.iter().fold(0, |sum, map| {
        let (row, col) = map.find_partial_reflections();

        sum + 100 * row.unwrap_or(0) + col.unwrap_or(0)
    });

    assert_eq!(result, 30844);
}
