use std::collections::HashMap;

use crate::map_2d::{Direction, PerimeterNode, Position};

type PositionDirection = (Position, Direction);

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => Direction::Right,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DigInstruction {
    direction: Direction,
    steps: isize,
    color: String,
}

impl DigInstruction {
    pub fn from_text(text: &str) -> Self {
        let components: Vec<&str> = text.split_whitespace().collect();

        let direction = Direction::from(*components.get(0).unwrap());
        let steps = components.get(1).unwrap().parse().unwrap();
        let color = components
            .get(2)
            .unwrap()
            .to_string()
            .chars()
            .skip(1)
            .take(7)
            .collect();

        DigInstruction {
            direction,
            steps,
            color,
        }
    }
}

pub fn parse_instructions(input: &str) -> Vec<DigInstruction> {
    input.lines().map(DigInstruction::from_text).collect()
}

pub fn apply_instructions(instructions: &Vec<DigInstruction>) -> Vec<PositionDirection> {
    let mut result: Vec<PositionDirection> = vec![];

    for instruction in instructions {
        let DigInstruction {
            direction, steps, ..
        } = instruction;

        let (increment_x, increment_y) = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let (previous, _) = result.pop().unwrap_or(((0, 0), Direction::Left));

        for i in 0..=*steps {
            result.push((
                (previous.0 + i * increment_x, previous.1 + i * increment_y),
                *direction,
            ));
        }
    }

    result
}

pub fn construct_path(positions: &Vec<PositionDirection>) -> HashMap<Position, PerimeterNode> {
    let mut result: HashMap<Position, PerimeterNode> = HashMap::new();
    let mut previous = positions.last().unwrap();

    for next in positions {
        let (_, previous_direction) = previous;
        let (position, direction) = *next;

        if result.get(&position).is_some() {
            continue;
        }

        let symbol = symbol_from_previous(previous_direction, &direction);

        let node = PerimeterNode::new(position, symbol);

        result.insert(position, node);

        previous = next;
    }

    result
}

fn symbol_from_previous(previous: &Direction, direction: &Direction) -> char {
    match previous {
        Direction::Right => match direction {
            Direction::Up => 'F',
            Direction::Right | Direction::Left => '-',
            Direction::Down => 'L',
        },
        Direction::Left => match direction {
            Direction::Up => 'L',
            Direction::Right | Direction::Left => '-',
            Direction::Down => 'F',
        },
        Direction::Up => match direction {
            Direction::Up | Direction::Down => '|',
            Direction::Right => 'F',
            Direction::Left => '7',
        },
        Direction::Down => match direction {
            Direction::Up | Direction::Down => '|',
            Direction::Right => 'L',
            Direction::Left => 'J',
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_instruction() {
        let input = "R 6 (#70c710)";

        let result = DigInstruction::from_text(input);

        assert_eq!(result.steps, 6);
        assert_eq!(result.direction, Direction::Right);
        assert_eq!(result.color, "#70c710");
    }
}
