use super::map_2d::{Direction, Position};

#[derive(Debug, Clone, Copy)]
pub struct QueueNode {
    pub cost: u32,
    pub position: Position,
    pub direction: Direction,
    pub steps: u32,
}

impl QueueNode {
    pub fn new(cost: u32, position: Position, direction: Direction, steps: u32) -> QueueNode {
        QueueNode {
            cost,
            position,
            direction,
            steps,
        }
    }

    pub fn score(&self) -> u32 {
        self.cost
    }
}

impl PartialEq for QueueNode {
    fn eq(&self, other: &Self) -> bool {
        self.score() == other.score()
    }
}

impl Eq for QueueNode {}

impl PartialOrd for QueueNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.score().partial_cmp(&(self.score()))
    }
}

impl Ord for QueueNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score().cmp(&(other.score()))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct StateKey {
    position: Position,
    direction: Direction,
    steps: u32,
}

impl From<QueueNode> for StateKey {
    fn from(value: QueueNode) -> Self {
        Self {
            position: value.position,
            direction: value.direction,
            steps: value.steps,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BinaryHeap;

    #[test]
    fn correctly_adds_and_finds_elements_with_cost() {
        let mut queue = BinaryHeap::new();

        let state_1 = QueueNode::new(1, (0, 0), Direction::E, 0);
        let state_2 = QueueNode::new(2, (0, 1), Direction::E, 0);
        let state_3 = QueueNode::new(1, (1, 0), Direction::E, 0);

        queue.push(state_1.clone());
        queue.push(state_2.clone());
        queue.push(state_3.clone());

        let found = queue.pop();
        assert_eq!(found, Some(state_1));

        let found = queue.pop();
        assert_eq!(found, Some(state_3));

        let found = queue.pop();
        assert_eq!(found, Some(state_2));
    }
}
