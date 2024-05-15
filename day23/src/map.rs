use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

pub type Position = (usize, usize);

#[derive(Debug, PartialEq, Clone)]
pub enum MapFeature {
    Slope(Direction),
    Empty,
    Blocked,
}

impl MapFeature {
    pub fn from_text(symbol: char) -> Self {
        match symbol {
            '^' => MapFeature::Slope(Direction::N),
            'v' => MapFeature::Slope(Direction::S),
            '<' => MapFeature::Slope(Direction::W),
            '>' => MapFeature::Slope(Direction::E),
            '.' => MapFeature::Empty,
            _ => MapFeature::Blocked,
        }
    }
}

// impl MapNode {
//     pub fn find_connected_positions(
//         &self,
//         from_direction: Direction,
//     ) -> Vec<(Direction, Position)> {
//         let (x, y) = self.position;

//         let mut positions = vec![(Direction::E, (x + 1, y)), (Direction::S, (x, y + 1))];

//         if x > 0 {
//             positions.push((Direction::W, (x - 1, y)))
//         }

//         if y > 0 {
//             positions.push((Direction::N, (x, y - 1)))
//         }

//         // avoid doubling back
//         positions
//             .into_iter()
//             .filter(|(direction, _)| match from_direction {
//                 Direction::E => direction != &Direction::W,
//                 Direction::W => direction != &Direction::E,
//                 Direction::N => direction != &Direction::S,
//                 Direction::S => direction != &Direction::N,
//                 _ => true,
//             })
//             .collect()
//     }
// }

#[derive(Debug)]
pub struct Map {
    nodes: HashMap<Position, MapFeature>,
    pub size: (usize, usize),
}

impl Map {
    pub fn from_text(text: &str) -> Map {
        let mut nodes: HashMap<Position, MapFeature> = HashMap::new();
        let mut size = (0, 0);

        for (y, line) in text.lines().enumerate() {
            for (x, symbol) in line.chars().enumerate() {
                let position = (x, y);

                let feature = MapFeature::from_text(symbol.to_string().parse().unwrap());

                size.0 = x;
                nodes.insert(position, feature);
            }

            size.1 = y;
        }

        Map { nodes, size }
    }
}

impl Map {
    pub fn get(&self, position: &Position) -> Option<&MapFeature> {
        self.nodes.get(position)
    }

    pub fn find_start_position(&self) -> Position {
        for i in 0..self.size.0 {
            let position = (i, 0);
            let node = self.get(&position).unwrap();

            if node == &MapFeature::Empty {
                return position;
            }
        }

        (0, 0)
    }

    pub fn find_end_position(&self) -> Position {
        for i in (0..self.size.0).rev() {
            let position = (i, self.size.1);
            let node = self.get(&position).unwrap();

            if node == &MapFeature::Empty {
                return position;
            }
        }

        (0, 0)
    }

    pub fn find_connecting_positions(&self, position: &Position) -> Vec<Position> {
        let positions = self.find_adjacent_positions(position);

        positions
            .iter()
            .filter_map(|position| {
                let node = self.get(position);
                match node {
                    None => None,
                    Some(node) => match node {
                        MapFeature::Blocked => None,
                        _ => Some(*position),
                    },
                }
            })
            .collect()
    }

    fn find_adjacent_positions(&self, position: &Position) -> Vec<Position> {
        let node = self.get(position).unwrap();
        let (x, y) = *position;

        match node {
            MapFeature::Slope(direction) => match direction {
                Direction::N => vec![(x, y - 1)],
                Direction::S => vec![(x, y + 1)],
                Direction::E => vec![(x + 1, y)],
                Direction::W => vec![(x - 1, y)],
            },
            _ => {
                let mut positions = vec![(x + 1, y), (x, y + 1)];

                if x > 0 {
                    positions.push((x - 1, y));
                }

                if y > 0 {
                    positions.push((x, y - 1));
                }

                positions
            }
        }
    }
}
