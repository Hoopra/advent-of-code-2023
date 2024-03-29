use std::fs::read_to_string;

use model::TerrainMap;

mod model;

fn main() {
    println!("Hello, world!");
    let file_path = "../input.txt";

    solve_part_1(file_path);
}

fn solve_part_1(file_path: &str) {
    let text = read_to_string(file_path).unwrap();

    let map_texts: Vec<&str> = text.split("\n\n").collect();

    let maps: Vec<TerrainMap> = map_texts
        .iter()
        .map(|text| TerrainMap::from_text(text))
        .collect();

    let result = maps.iter().fold(0, |sum, map| {
        let row = map.find_reflecting_row();
        let col = map.find_reflecting_col();

        sum + 100 * row.unwrap_or(0) + col.unwrap_or(0)
    });

    assert_eq!(result, 30535);
}
