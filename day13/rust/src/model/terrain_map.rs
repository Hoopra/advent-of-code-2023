use std::{collections::HashMap, vec};

use super::{Feature, TerrainCollection};

type Position = (usize, usize);

type RowPair = Position;

pub struct TerrainMap {
    layout: HashMap<Position, Feature>,
    size: (usize, usize),
}

impl TerrainMap {
    pub fn from_text(text: &str) -> TerrainMap {
        let mut layout = HashMap::new();
        let mut size = (0, 0);

        for (y, line) in text.lines().enumerate() {
            for (x, symbol) in line.chars().enumerate() {
                let feature = match symbol {
                    '.' => Feature::Ash,
                    _ => Feature::Rock,
                };

                layout.insert((x, y), feature);
                size.0 = x + 1;
            }

            size.1 = y + 1;
        }

        TerrainMap { layout, size }
    }
}

impl TerrainMap {
    fn rows(&self) -> Vec<TerrainCollection> {
        let mut result = vec![];
        let (x_max, y_max) = self.size;

        for y in 0..y_max {
            let mut row = vec![];

            for x in 0..x_max {
                let feature = self.layout.get(&(x, y));
                row.push(*feature.unwrap());
            }

            result.push(TerrainCollection::new(row));
        }

        result
    }

    fn cols(&self) -> Vec<TerrainCollection> {
        let mut result = vec![];
        let (x_max, y_max) = self.size;

        for x in 0..x_max {
            let mut col = vec![];

            for y in 0..y_max {
                let feature = self.layout.get(&(x, y));
                col.push(*feature.unwrap());
            }

            result.push(TerrainCollection::new(col));
        }

        result
    }

    fn find_row_pairs(&self) -> Vec<RowPair> {
        find_pairs(&self.rows())
    }

    fn find_col_pairs(&self) -> Vec<RowPair> {
        find_pairs(&self.cols())
    }

    pub fn find_reflecting_row(&self) -> Option<usize> {
        find_reflection(&self.find_row_pairs(), self.size.1 - 1)
    }

    pub fn find_reflecting_col(&self) -> Option<usize> {
        find_reflection(&self.find_col_pairs(), self.size.0 - 1)
    }
}

fn find_reflection(pairs: &Vec<RowPair>, max_index: usize) -> Option<usize> {
    let candidate = pairs.iter().find(|(index_a, index_b)| {
        if index_b - index_a != 1 {
            return false;
        }

        is_reflection(*index_a, pairs, max_index)
    });

    match candidate {
        Some((_, value)) => Some(*value),
        None => None,
    }
}

fn is_reflection(index: usize, pairs: &Vec<RowPair>, max_index: usize) -> bool {
    for i in 1..(max_index - index) {
        let upper = index + i + 1;

        if i > index || upper > max_index {
            continue;
        }

        if !pairs.contains(&(index - i, upper)) {
            return false;
        }
    }

    true
}

fn find_pairs(collection: &Vec<TerrainCollection>) -> Vec<RowPair> {
    let mut result = vec![];
    let num = collection.len();

    for i in 0..num {
        let first_row = collection.get(i).unwrap();

        for j in (i + 1)..num {
            let second_row = collection.get(j).unwrap();
            if first_row == second_row {
                result.push((i, j));
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_terrain_map() {
        let input = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.";

        let map = TerrainMap::from_text(input);

        assert_eq!(map.size, (9, 7));
    }

    #[test]
    fn finds_map_rows_cols() {
        let input = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.";

        let map = TerrainMap::from_text(input);

        let rows = map.rows();
        let cols = map.cols();

        assert_eq!(rows.len(), 7);
        assert_eq!(cols.len(), 9);
    }

    #[test]
    fn finds_paired_rows() {
        let input = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.";

        let map = TerrainMap::from_text(input);

        assert_eq!(map.find_row_pairs(), vec![(1, 4), (2, 3)]);
    }

    #[test]
    fn finds_paired_cols() {
        let input = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.";

        let map = TerrainMap::from_text(input);

        assert_eq!(map.find_col_pairs(), vec![(1, 8), (2, 7), (3, 6), (4, 5)]);
    }

    #[test]
    fn finds_reflections() {
        let input = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.";

        let map = TerrainMap::from_text(input);

        assert_eq!(map.find_reflecting_row(), None);
        assert_eq!(map.find_reflecting_col(), Some(5));

        let input = "#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#";

        let map = TerrainMap::from_text(input);

        assert_eq!(map.find_reflecting_row(), Some(4));
        assert_eq!(map.find_reflecting_col(), None);
    }
}
