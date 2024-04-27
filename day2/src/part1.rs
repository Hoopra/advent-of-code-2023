use crate::cube::GameState;
use crate::parse_text::text_to_game;
use std::fs::read_to_string;

pub fn solve_part_1(file_path: &str) {
    let file = read_to_string(file_path).unwrap();

    let state = GameState::new(12, 13, 14);
    let mut id_sum = 0;

    for line in file.lines() {
        let game = text_to_game(line);

        let highest_draw = game.find_maximum_draws();

        let possible = state.is_game_draw_possible(&highest_draw);
        if possible {
            id_sum += game.id;
        }
    }

    println!("sum of possible game ids: {id_sum}")
}
