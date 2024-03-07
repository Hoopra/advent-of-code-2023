use crate::parse_text::text_to_game;
use std::fs::read_to_string;

pub fn solve_part_2(file_path: &str) {
    let file = read_to_string(file_path).unwrap();

    let mut minimum_cube_sum = 0;

    for line in file.lines() {
        let game = text_to_game(line);

        let highest_draw = game.find_maximum_draws();

        let cube = highest_draw.cube();

        minimum_cube_sum += cube;
    }

    println!("sum of minimum cubes: {minimum_cube_sum}")
}
