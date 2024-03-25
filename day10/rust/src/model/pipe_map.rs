use std::collections::HashMap;

use crate::model::{Pipe, Position, SurroundingTiles};

type PipeTiles = HashMap<Position, Pipe>;

pub struct PipeMap {
    pub start: Position,
    tiles: PipeTiles,

    pub size: (usize, usize),
}

impl PipeMap {
    pub fn from_text(input: &str) -> PipeMap {
        let mut start_x = 0;
        let mut start_y = 0;

        let size = (
            input
                .lines()
                .collect::<Vec<&str>>()
                .get(0)
                .unwrap()
                .chars()
                .count(),
            input.lines().count(),
        );

        let mut tiles = HashMap::new();

        for (y, line) in input.lines().enumerate() {
            for (x, symbol) in line.chars().enumerate() {
                if symbol == '.' {
                    continue;
                }

                if symbol == 'S' {
                    start_x = x;
                    start_y = y;
                }

                let position = (x, y);
                tiles.insert(position, Pipe::new(position, symbol));
            }
        }

        let start = (start_x, start_y);

        let mut map = PipeMap { tiles, start, size };

        map.set_pipe(&start, find_starting_pipe(&map, start));

        map
    }
}

impl PipeMap {
    pub fn get_tile(&self, position: &Position) -> Option<&Pipe> {
        self.tiles.get(position)
    }

    pub fn set_pipe(&mut self, position: &Position, pipe: Pipe) {
        self.tiles.insert(*position, pipe);
    }
}

fn find_starting_pipe(map: &PipeMap, position: Position) -> Pipe {
    let SurroundingTiles {
        left,
        right,
        up,
        down,
    } = find_surrounding_tiles(map, position);

    Pipe::from_connections(
        position,
        left.is_some() && left.as_ref().unwrap().connects_right(),
        right.is_some() && right.as_ref().unwrap().connects_left(),
        up.is_some() && up.as_ref().unwrap().connects_down(),
        down.is_some() && down.as_ref().unwrap().connects_up(),
    )
}

fn find_surrounding_tiles(map: &PipeMap, position: Position) -> SurroundingTiles {
    let (x, y) = position;

    SurroundingTiles {
        left: match x == 0 {
            true => None,
            false => map.get_tile(&(x - 1, y)),
        },
        right: map.get_tile(&(x + 1, y)),
        up: match y == 0 {
            true => None,
            false => map.get_tile(&(x, y - 1)),
        },
        down: map.get_tile(&(x, y + 1)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_map_from_text() {
        let input = ".....\n.S-7.\n.|.|.\n.L-J.\n.....";

        let map = PipeMap::from_text(&input);

        assert_eq!(map.start, (1, 1));

        assert_eq!(map.get_tile(&(0, 1)), None);
        assert_eq!(
            map.get_tile(&(1, 1)),
            Some(&Pipe::from_connections((1, 1), false, true, false, true))
        );
    }

    #[test]
    fn finds_correct_tile_in_map() {
        let input = "...........\n.S-------7.\n.|F-----7|.\n.||.....||.\n.||.....||.\n.|L-7.F-J|.\n.|..|.|..|.\n.L--J.L--J.\n...........";

        let map = PipeMap::from_text(&input);

        assert_eq!(map.get_tile(&(2, 6)), None);
    }
}
