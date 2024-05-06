use std::{
    collections::{HashMap, HashSet},
    usize,
};

type Position = (usize, usize);

#[derive(Debug, Clone, Copy)]
pub enum MapFeature {
    Empty,
    Blocked,
}

#[derive(Debug)]
pub struct Map {
    features: HashMap<Position, MapFeature>,
    start: Position,
}

impl Map {
    pub fn from_text(text: &str) -> Map {
        let mut features = HashMap::new();
        let mut start = (0, 0);

        text.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, symbol)| {
                let feature = match symbol {
                    '.' => MapFeature::Empty,
                    'S' => {
                        start = (x, y);
                        MapFeature::Empty
                    }
                    _ => MapFeature::Blocked,
                };

                features.insert((x, y), feature);
            })
        });

        Map { features, start }
    }
}

impl Map {
    pub fn get(&self, position: &Position) -> MapFeature {
        *self.features.get(position).unwrap_or(&MapFeature::Blocked)
    }

    pub fn neighboring_positions(&self, position: &Position) -> Vec<Position> {
        let (x, y) = *position;
        let mut neighbors = vec![];

        if x > 0 {
            neighbors.push((x - 1, y));
        }

        if y > 0 {
            neighbors.push((x, y - 1));
        }

        neighbors.push((x, y + 1));
        neighbors.push((x + 1, y));

        neighbors
    }

    pub fn accessible_neighboring_positions(&self, position: &Position) -> Vec<Position> {
        self.neighboring_positions(position)
            .iter()
            .filter_map(|position| match self.get(position) {
                MapFeature::Empty => Some(*position),
                _ => None,
            })
            .collect()
    }

    pub fn reachable_positions(&self, steps: usize) -> usize {
        let start = self.start;
        let mut current: HashSet<Position> = HashSet::from([start]);

        for _ in 1..=steps {
            let positions = current.clone();
            current.clear();

            for position in positions {
                self.accessible_neighboring_positions(&position)
                    .iter()
                    .for_each(|position| {
                        current.insert(*position);
                    });
            }
        }

        current.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Map;

    #[test]
    fn calculates_reachable_positions() {
        let text = "...........\n.....###.#.\n.###.##..#.\n..#.#...#..\n....#.#....\n.##..S####.\n.##..#...#.\n.......##..\n.##.#.####.\n.##..##.##.\n...........";
        let map = Map::from_text(text);

        assert_eq!(map.reachable_positions(6), 16);
    }
}
