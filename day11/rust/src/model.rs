pub type Position = (usize, usize);

type MapSize = Position;

pub struct StarMap {
    galaxies: Vec<Position>,
    size: MapSize,
}

impl StarMap {
    pub fn new(input: &str) -> StarMap {
        let mut galaxies = vec![];

        let mut size = (0, 0);

        for (y, line) in input.lines().enumerate() {
            size.0 = y;

            for (x, char) in line.chars().enumerate() {
                size.1 = x;

                match char {
                    '#' => galaxies.push((x, y)),
                    _ => {}
                }
            }
        }

        StarMap { galaxies, size }
    }
}

impl StarMap {
    pub fn expand(&mut self) {
        let (x_empty, y_empty) = self.find_empty_rows_cols();
        self.expand_rows(x_empty);
        self.expand_cols(y_empty);
    }

    fn find_empty_rows_cols(&self) -> (Vec<usize>, Vec<usize>) {
        let (x_filled, y_filled) =
            self.galaxies
                .iter()
                .fold((vec![], vec![]), |accumulator, (x, y)| {
                    let (mut rows, mut cols) = accumulator;

                    rows.push(*y);
                    cols.push(*x);

                    (rows, cols)
                });

        let x_empty = (0..self.size.0)
            .filter(|value| !x_filled.contains(value))
            .collect();

        let y_empty = (0..self.size.1)
            .filter(|value| !y_filled.contains(value))
            .collect();

        (x_empty, y_empty)
    }

    pub fn expand_rows(&mut self, indexes: Vec<usize>) {
        self.galaxies = self
            .galaxies
            .iter()
            .map(|position| {
                let (x, y) = position;

                let previous_rows = indexes
                    .iter()
                    .filter(|index| index < &y)
                    .collect::<Vec<&usize>>()
                    .len();

                println!("galaxy {:?}, expand y {} times", position, previous_rows);

                (*x, *y + previous_rows)
            })
            .collect();

        self.size.0 += 1;
    }

    pub fn expand_cols(&mut self, indexes: Vec<usize>) {
        self.galaxies = self
            .galaxies
            .iter()
            .map(|position| {
                let (x, y) = position;

                let previous_cols = indexes
                    .iter()
                    .filter(|index| index < &x)
                    .collect::<Vec<&usize>>()
                    .len();

                println!("galaxy {:?}, expand x {} times", position, previous_cols);

                (*x + previous_cols, *y)
            })
            .collect();

        self.size.1 += 1;
    }

    pub fn find_galaxy_pairs(&self) -> Vec<(&Position, &Position)> {
        let mut result = vec![];

        let total_galaxies = self.galaxies.len();

        for i in 0..total_galaxies {
            let first_position = self.galaxies.get(i);

            for j in (i + 1)..total_galaxies {
                let second_position = self.galaxies.get(j);
                result.push((first_position.unwrap(), second_position.unwrap()));
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expands_galaxy_row() {
        let input = "#.........\n..........\n......#...";

        let mut map = StarMap::new(input);

        assert_eq!(map.galaxies, vec![(0, 0), (6, 2)]);

        map.expand_cols(vec![1]);
        map.expand_rows(vec![1]);

        assert_eq!(map.galaxies, vec![(0, 0), (7, 3)]);
    }

    #[test]
    fn finds_galaxy_pairs() {
        let input = "......#...\n.#........\n.........#\n..........\n.......#..";

        let map = StarMap::new(input);
        let pairs = map.find_galaxy_pairs();

        assert_eq!(pairs.len(), 6);
    }

    #[test]
    fn finds_empty_rows_and_columns() {
        let input = "......#...\n.#........\n.........#\n..........\n.......#..";

        let map = StarMap::new(input);
        let empty = map.find_empty_rows_cols();

        assert_eq!(empty.0, vec![3]);
        assert_eq!(empty.1, vec![0, 2, 3, 4, 5, 8]);
    }

    #[test]
    fn expands_map() {
        let input = "......#...\n.#........\n.........#\n..........\n.......#..";
        let mut map = StarMap::new(input);

        assert_eq!(map.galaxies, vec![(6, 0), (1, 1), (9, 2), (7, 4)]);

        map.expand();

        assert_eq!(map.galaxies, vec![(11, 0), (2, 1), (15, 2), (12, 5)]);
    }
}
