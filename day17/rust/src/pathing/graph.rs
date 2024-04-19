use std::collections::HashMap;

use super::map_2d::{Direction, Position};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct GraphNode {
    pub position: Position,
    pub value: u32,
}

impl GraphNode {
    pub fn new(position: Position, value: u32) -> GraphNode {
        GraphNode { position, value }
    }
}

impl GraphNode {
    pub fn find_connected_positions(
        &self,
        from_direction: Direction,
    ) -> Vec<(Direction, Position)> {
        let (x, y) = self.position;

        let mut positions = vec![(Direction::E, (x + 1, y)), (Direction::S, (x, y + 1))];

        if x > 0 {
            positions.push((Direction::W, (x - 1, y)))
        }

        if y > 0 {
            positions.push((Direction::N, (x, y - 1)))
        }

        // avoid doubling back
        positions
            .into_iter()
            .filter(|(direction, _)| match from_direction {
                Direction::E => direction != &Direction::W,
                Direction::W => direction != &Direction::E,
                Direction::N => direction != &Direction::S,
                Direction::S => direction != &Direction::N,
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct Graph {
    nodes: HashMap<Position, GraphNode>,
    pub size: (usize, usize),
}

impl Graph {
    pub fn from_text(text: &str) -> Graph {
        let mut nodes: HashMap<Position, GraphNode> = HashMap::new();
        let mut size = (0, 0);

        for (y, line) in text.lines().enumerate() {
            for (x, symbol) in line.chars().enumerate() {
                let position = (x, y);

                let node = GraphNode::new(position, symbol.to_string().parse().unwrap());

                size.0 = x;
                nodes.insert(position, node);
            }

            size.1 = y;
        }

        Graph { nodes, size }
    }
}

impl Graph {
    pub fn get(&self, position: &Position) -> Option<&GraphNode> {
        self.nodes.get(position)
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_all_connecting_positions() {
        let node = GraphNode::new((5, 5), 5);

        let positions = node.find_connected_positions(Direction::E);

        assert_eq!(
            positions,
            vec![
                (Direction::E, (6, 5)),
                (Direction::S, (5, 6)),
                (Direction::N, (5, 4))
            ]
        );
    }
}
