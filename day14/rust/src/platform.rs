use super::cycle::find_repeating_pattern;
use std::collections::HashMap;

type Position = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TerrainFeature {
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
        self.tilt(&Platform::find_round_rock_end_north)
    }

    fn tilt_south(&mut self) {
        self.tilt_reverse(&Platform::find_round_rock_end_south)
    }

    fn tilt_west(&mut self) {
        self.tilt(&Platform::find_round_rock_end_west)
    }

    fn tilt_east(&mut self) {
        self.tilt_reverse(&Platform::find_round_rock_end_east)
    }

    pub fn perform_cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    pub fn calculate_load_after_cycles(&mut self, cycles: usize) -> usize {
        let mut loads = vec![];
        let mut pattern: Option<(usize, usize)> = None;
        let mut index = 0;

        while pattern.is_none() && index < cycles {
            for _ in 0..50 {
                self.perform_cycle();
                index += 1;

                loads.push(self.calculate_north_load());
            }

            pattern = find_repeating_pattern(&loads);
        }

        let (offset, period) = pattern.unwrap();

        let remainder = (cycles - offset) % period;
        let index = (period - 1 + remainder) % period;

        let pattern = &loads[offset..(offset + period)];

        let result = pattern.get(index);

        *result.unwrap()
    }

    fn tilt(&mut self, find_end_position: &dyn Fn(&Platform, &Position) -> Position) {
        let (x_max, y_max) = self.size;

        for x in 0..x_max {
            for y in 0..y_max {
                let start_position = (x, y);
                self.move_rock(start_position, find_end_position);
            }
        }
    }

    fn tilt_reverse(&mut self, find_end_position: &dyn Fn(&Platform, &Position) -> Position) {
        let (x_max, y_max) = self.size;

        for x in (0..x_max).rev() {
            for y in (0..y_max).rev() {
                let start_position = (x, y);
                self.move_rock(start_position, find_end_position);
            }
        }
    }

    fn move_rock(
        &mut self,
        start_position: Position,
        find_end_position: &dyn Fn(&Platform, &Position) -> Position,
    ) {
        let tile = self.tiles.get(&start_position).unwrap();

        match tile {
            TerrainFeature::RoundRock => {
                let end_position = find_end_position(&self, &start_position);

                if end_position == start_position {
                    return;
                }

                self.tiles.insert(start_position, TerrainFeature::Empty);
                self.tiles.insert(end_position, TerrainFeature::RoundRock);
            }
            _ => {}
        };
    }

    fn find_round_rock_end_north(&self, start: &Position) -> Position {
        self.find_round_rock_end_north_south(start, 0, &|y| y - 1)
    }

    fn find_round_rock_end_south(&self, start: &Position) -> Position {
        self.find_round_rock_end_north_south(start, self.size.1 - 1, &|y| y + 1)
    }

    fn find_round_rock_end_west(&self, start: &Position) -> Position {
        self.find_round_rock_end_west_east(start, 0, &|x| x - 1)
    }

    fn find_round_rock_end_east(&self, start: &Position) -> Position {
        self.find_round_rock_end_west_east(start, self.size.0 - 1, &|x| x + 1)
    }

    fn find_round_rock_end_north_south(
        &self,
        start: &Position,
        y_end: usize,
        increment_y: &dyn Fn(usize) -> usize,
    ) -> Position {
        let (x_start, y_start) = start;
        let mut end = *start;
        let mut y = *y_start;

        while y != y_end {
            y = increment_y(y);
            let position = (*x_start, y);
            let tile = self.tiles.get(&position).unwrap();

            match tile {
                TerrainFeature::Empty => end = position,
                _ => break,
            }
        }

        end
    }

    fn find_round_rock_end_west_east(
        &self,
        start: &Position,
        x_end: usize,
        increment_x: &dyn Fn(usize) -> usize,
    ) -> Position {
        let (x_start, y_start) = start;
        let mut end = *start;
        let mut x = *x_start;

        while x != x_end {
            x = increment_x(x);
            let position = (x, *y_start);
            let tile = self.tiles.get(&position).unwrap();

            match tile {
                TerrainFeature::Empty => end = position,
                _ => break,
            }
        }

        end
    }

    pub fn calculate_north_load(&self) -> usize {
        let max_distance = &self.size.1;

        self.tiles
            .iter()
            .fold(0, |sum, ((_, y), feature)| match feature {
                TerrainFeature::RoundRock => sum + max_distance - y,
                _ => sum,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn calculates_load_after_cycles() {
        let input = "O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....";

        let mut platform = Platform::from_text(input);

        let result = platform.calculate_load_after_cycles(15);
        assert_eq!(result, 63);

        let mut platform = Platform::from_text(input);

        let result = platform.calculate_load_after_cycles(1_000_000_000);
        assert_eq!(result, 64);
    }
}
