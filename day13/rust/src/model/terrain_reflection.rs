use crate::util::find_reflection;

use super::{RowPair, TerrainCollection, TerrainMap};

impl TerrainMap {
    pub fn find_row_pairs(&self) -> Vec<RowPair> {
        find_pairs(&self.rows())
    }

    pub fn find_col_pairs(&self) -> Vec<RowPair> {
        find_pairs(&self.cols())
    }

    pub fn find_reflecting_row(&self) -> Option<usize> {
        find_reflection(&self.find_row_pairs(), self.size().1 - 1)
    }

    pub fn find_reflecting_col(&self) -> Option<usize> {
        find_reflection(&self.find_col_pairs(), self.size().0 - 1)
    }
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
