use std::collections::HashMap;

type Position = (usize, usize);

#[derive(Debug)]
enum TerrainFeature {
    Empty,
    RoundRock,
    CubeRock,
}

pub struct Platform {
    tiles: HashMap<Position, TerrainFeature>,
    size: (usize, usize),
}

impl Platform {
    pub fn from_text(text: &str) -> Platform {
        let mut tiles = HashMap::new();
        let mut size = (0, 0);

        for (y, line) in text.lines().enumerate() {
            for (x, symbol) in line.chars().enumerate() {
                let feature = match symbol {
                    '.' => TerrainFeature::Empty,
                    '#' => TerrainFeature::CubeRock,
                    'O' | _ => TerrainFeature::RoundRock,
                };

                tiles.insert((x, y), feature);
                size.0 = x + 1;
            }

            size.1 = y + 1;
        }

        Platform { tiles, size }
    }
}

impl Platform {
    pub fn tilt_north(&mut self) {
        let (x_max, y_max) = self.size;

        for x in 0..x_max {
            for y in 0..y_max {
                let start_position = (x, y);
                let tile = self.tiles.get(&start_position).unwrap();

                match tile {
                    TerrainFeature::RoundRock => {
                        let end_position = self.find_round_rock_end_north(&start_position);

                        if end_position == start_position {
                            continue;
                        }

                        self.tiles.insert(start_position, TerrainFeature::Empty);
                        self.tiles.insert(end_position, TerrainFeature::RoundRock);
                    }
                    _ => {}
                };
            }
        }
    }

    fn find_round_rock_end_north(&self, start: &Position) -> Position {
        let (x_start, y_start) = start;
        let mut end = *start;
        let mut y = *y_start;

        while y > 0 {
            y -= 1;
            let position = (*x_start, y);
            let tile = self.tiles.get(&position).unwrap();

            match tile {
                TerrainFeature::Empty => end = position,
                _ => break,
            }
        }

        end
    }

    pub fn calculate_north_load(&self) -> u32 {
        let max_distance = &self.size.1;

        self.tiles
            .iter()
            .fold(0, |sum, ((_, y), feature)| match feature {
                TerrainFeature::RoundRock => sum + (max_distance - y) as u32,
                _ => sum,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::Platform;

    #[test]
    fn calculates_load_before_tilt() {
        let input = "O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....";

        let platform = Platform::from_text(input);

        assert_eq!(platform.calculate_north_load(), 104);
    }

    #[test]
    fn calculates_load_after_tilt() {
        let input = "O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....";

        let mut platform = Platform::from_text(input);

        platform.tilt_north();

        assert_eq!(platform.calculate_north_load(), 136);
    }
}
