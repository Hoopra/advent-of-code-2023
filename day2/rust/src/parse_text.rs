use crate::cube::{CubeDraw, CubeGame};

pub fn text_to_game(text: &str) -> CubeGame {
    let (name, draws) = text.split_at(text.find(": ").unwrap() + 2);
    let id = parse_game_id(name).unwrap();

    let draws = draws.split("; ").map(parse_draw).collect();

    CubeGame::new(id, draws)
}

fn parse_game_id(name: &str) -> Option<u16> {
    let index_start = name.find(' ');
    let index_end = name.find(':');

    if index_start.is_none() || index_end.is_none() {
        return None;
    }

    let id = &name[(index_start.unwrap() + 1)..index_end.unwrap()];

    id.parse().ok()
}

fn parse_draw(input: &str) -> CubeDraw {
    let draws = input
        .split(", ")
        .map(parse_color_draw)
        .fold(CubeDraw::default(), |previous, next| {
            previous.combine_with(next)
        });

    draws
}

fn parse_color_draw(input: &str) -> CubeDraw {
    let (number, color) = input.split_at(input.find(' ').unwrap());

    let number = number.parse().unwrap();

    match color.trim() {
        "green" => CubeDraw::new(0, number, 0),
        "blue" => CubeDraw::new(0, 0, number),
        _ => CubeDraw::new(number, 0, 0),
    }
}

#[cfg(test)]
mod test_find_digit {
    use super::*;

    #[test]
    fn correctly_evaluates_game_from_line() {
        let text_line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = text_to_game(text_line);

        assert_eq!(game.id, 1);

        let higest_draws = game.find_maximum_draws();

        assert_eq!(higest_draws.red, 4);
        assert_eq!(higest_draws.blue, 6);
        assert_eq!(higest_draws.green, 2);

        let text_line = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let game = text_to_game(text_line);

        assert_eq!(game.id, 4);

        let higest_draws = game.find_maximum_draws();

        assert_eq!(higest_draws.red, 14);
        assert_eq!(higest_draws.blue, 15);
        assert_eq!(higest_draws.green, 3);
    }
}
