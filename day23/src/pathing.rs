use crate::map::{Map, Position};

#[derive(Debug, Clone)]
struct Path {
    visited: Vec<Position>,
}

impl Path {
    pub fn new(visited: Vec<Position>) -> Self {
        Self { visited }
    }
}

pub fn find_longest_path_in_map(map: &Map, start: &Position, end: &Position) -> usize {
    let mut paths: Vec<Path> = vec![Path::new(vec![*start])];
    let mut settled_paths: Vec<Path> = vec![];

    while paths.len() > 0 {
        let path = paths.pop().unwrap();

        let tip = path.visited.last().unwrap();

        let neighbors: Vec<Position> = map
            .find_connecting_positions(tip)
            .iter()
            .filter_map(|position| match path.visited.contains(&position) {
                true => None,
                _ => Some(*position),
            })
            .collect();

        if neighbors.len() == 0 {
            continue;
        }

        neighbors.iter().for_each(|position| {
            let mut next_path = path.clone();
            next_path.visited.push(*position);

            if position == end {
                settled_paths.push(next_path);
            } else {
                paths.push(next_path);
            }
        })
    }

    settled_paths.iter().fold(0, |result, path| {
        let length = path.visited.len() - 1;
        if length > result {
            length
        } else {
            result
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_longest_path_in_test_map() {
        let text = "##.###\n#.>..#\n###.##\n#...##\n#.####";
        let graph = Map::from_text(&text);

        let start = graph.find_start_position();
        let end = graph.find_end_position();

        let result = find_longest_path_in_map(&graph, &start, &end);
        assert_eq!(result, 7);
    }

    #[test]
    fn finds_longest_path_in_map() {
        let text = "#.#####################\n#.......#########...###\n#######.#########.#.###\n###.....#.>.>.###.#.###\n###v#####.#v#.###.#.###\n###.>...#.#.#.....#...#\n###v###.#.#.#########.#\n###...#.#.#.......#...#\n#####.#.#.#######.#.###\n#.....#.#.#.......#...#\n#.#####.#.#.#########v#\n#.#...#...#...###...>.#\n#.#.#v#######v###.###v#\n#...#.>.#...>.>.#.###.#\n#####v#.#.###v#.#.###.#\n#.....#...#...#.#.#...#\n#.#########.###.#.#.###\n#...###...#...#...#.###\n###.###.#.###v#####v###\n#...#...#.#.>.>.#.>.###\n#.###.###.#.###.#.#v###\n#.....###...###...#...#\n#####################.#";
        let graph = Map::from_text(&text);

        let start = graph.find_start_position();
        let end = graph.find_end_position();

        let result = find_longest_path_in_map(&graph, &start, &end);
        assert_eq!(result, 94);
    }
}
