mod engine;

use engine::Engine;
use std::fs::read_to_string;

fn main() {
    let file_path = "../input.txt";
    let schematic = read_to_string(file_path).unwrap();

    let engine = Engine::new(schematic);

    let part_score = engine.calculate_part_score();

    println!("grand total: {part_score}");
    assert_eq!(part_score, 528819);

    let gear_ratio = engine.calculate_gear_ratio();

    println!("grand total: {gear_ratio}");
    assert_eq!(gear_ratio, 80403602);
}
