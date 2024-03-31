use std::collections::HashMap;

use super::{Position, TerrainCollection, TerrainFeature};

pub struct TerrainMap {
    layout: HashMap<Position, TerrainFeature>,
    size: (usize, usize),
}

impl TerrainMap {
    pub fn from_text(text: &str) -> TerrainMap {
        let mut layout = HashMap::new();
        let mut size = (0, 0);

        for (y, line) in text.lines().enumerate() {
            for (x, symbol) in line.chars().enumerate() {
                let feature = match symbol {
                    '.' => TerrainFeature::Ash,
                    _ => TerrainFeature::Rock,
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
    pub fn rows(&self) -> Vec<TerrainCollection> {
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

    pub fn cols(&self) -> Vec<TerrainCollection> {
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

    pub fn size(&self) -> (usize, usize) {
        self.size
    }
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
}
