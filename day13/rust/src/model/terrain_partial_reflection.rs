use crate::util::find_reflections;

use super::{RowPair, TerrainCollection, TerrainMap};

impl TerrainMap {
    fn find_partial_row_pairs(&self) -> Vec<RowPair> {
        find_partial_pairs(&self.rows())
    }

    fn find_partial_col_pairs(&self) -> Vec<RowPair> {
        find_partial_pairs(&self.cols())
    }

    pub fn find_partial_reflections(&self) -> (Option<usize>, Option<usize>) {
        let rows = self.find_partial_reflecting_rows();
        let cols = self.find_partial_reflecting_cols();

        let row = rows.get(0).map(|row| *row);
        let col = cols.get(0).map(|col| *col);

        (row, col)
    }

    pub fn find_partial_reflecting_rows(&self) -> Vec<usize> {
        let rows = find_partial_reflections(
            &self.find_row_pairs(),
            &self.find_partial_row_pairs(),
            self.size().1 - 1,
        );

        let original_row = self.find_reflecting_row().unwrap_or(100);

        let filtered = rows
            .iter()
            .filter_map(|row| match row != &original_row {
                true => Some(*row),
                false => None,
            })
            .collect();

        // println!(
        //     "rows: {:?}, original: {}, filtered: {:?}",
        //     rows, original_row, filtered
        // );

        filtered
    }

    pub fn find_partial_reflecting_cols(&self) -> Vec<usize> {
        let cols = find_partial_reflections(
            &self.find_col_pairs(),
            &self.find_partial_col_pairs(),
            self.size().0 - 1,
        );

        let original_col = self.find_reflecting_col().unwrap_or(100);

        let filtered = cols
            .iter()
            .filter_map(|col| match col != &original_col {
                true => Some(*col),
                false => None,
            })
            .collect();

        // println!(
        //     "cols: {:?}, original: {}, filtered: {:?}",
        //     cols, original_col, filtered
        // );

        filtered
    }
}

fn find_partial_pairs(collection: &Vec<TerrainCollection>) -> Vec<RowPair> {
    let mut result = vec![];
    let num = collection.len();

    for i in 0..num {
        let first_row = collection.get(i).unwrap();

        for j in (i + 1)..num {
            let second_row = collection.get(j).unwrap();

            let differences = first_row.find_differences(second_row);
            if differences.len() == 1 {
                result.push((i, j));
            }
        }
    }

    result
}

fn find_partial_reflections(
    pairs: &Vec<RowPair>,
    partial_pairs: &Vec<RowPair>,
    max_index: usize,
) -> Vec<usize> {
    let candidates: Vec<usize> = partial_pairs
        .iter()
        .flat_map(|partial_row| {
            let (first_partial, second_partial) = partial_row;

            let mut current_pairs: Vec<RowPair> = vec![*partial_row];

            for row in pairs {
                let (first, second) = row;
                if first == first_partial || first == second_partial {
                    continue;
                }

                if second == first_partial || second == second_partial {
                    continue;
                }

                current_pairs.push(*row);
            }

            find_reflections(&current_pairs, max_index)
        })
        .collect();

    candidates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_partial_pairs() {
        let input = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.";

        let map = TerrainMap::from_text(input);

        assert_eq!(map.find_partial_row_pairs(), vec![(0, 5), (1, 6), (4, 6)]);
        assert_eq!(map.find_partial_col_pairs(), vec![]);

        let input = "#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#";

        let map = TerrainMap::from_text(input);

        assert_eq!(map.find_partial_row_pairs(), vec![(0, 1), (0, 6)]);
        assert_eq!(map.find_partial_col_pairs(), vec![(1, 4)]);
    }

    #[test]
    fn finds_partial_reflections() {
        let input = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.";

        let map = TerrainMap::from_text(input);

        let (row, col) = map.find_partial_reflections();
        assert_eq!(row, Some(3));
        assert_eq!(col, None);

        let input = "#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#";

        let map = TerrainMap::from_text(input);

        let (row, col) = map.find_partial_reflections();
        assert_eq!(row, Some(1));
        assert_eq!(col, None);
    }

    #[test]
    fn finds_partial_reflections_in_large_map() {
        let input = ".####.#...##.#.\n...###.##.#.#..\n...###.##.#.#..\n.####.#...#..#.\n..##......#..#.\n#.#...#.##...##\n.#.#.#..##..##.\n#..##...#####.#\n.#.#.#.#..##.#.\n.....###.#.#..#\n.....###.#.#..#";

        let map = TerrainMap::from_text(input);

        let (row, col) = map.find_partial_reflections();

        assert_eq!(row, Some(2));
        assert_eq!(col, None);
    }
}
