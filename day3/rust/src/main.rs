mod engine;

use engine::Engine;
use std::fs::read_to_string;

fn main() {
    let file_path = "../input.txt";
    let schematic = read_to_string(file_path).unwrap();

    let score = get_engine_part_score(schematic);

    println!("grand total: {score}");
    assert_eq!(score, 528819);
}

fn get_engine_part_score(schematic: String) -> u32 {
    let engine = Engine::new(schematic);

    engine.calculate_part_score()
}

#[cfg(test)]
mod test_engine_score {
    use super::*;

    #[test]
    fn calculates_part_score_for_engine() {
        let schematic = String::from("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..");

        assert_eq!(get_engine_part_score(schematic), 4361);
    }
}
