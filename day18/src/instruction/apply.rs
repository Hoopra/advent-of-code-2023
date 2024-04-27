use crate::position_2d::{Direction, Position};

use super::DigInstruction;

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

pub fn instructions_to_vertices(instructions: &Vec<DigInstruction>) -> Vec<Position> {
    let mut result: Vec<Position> = vec![];

    for instruction in instructions {
        let DigInstruction { direction, steps } = instruction;

        let (increment_x, increment_y) = direction_to_increment(direction);

        let (x, y) = result.last().unwrap_or(&(0, 0));

        result.push((x + steps * increment_x, y + steps * increment_y));
    }

    result
}

fn direction_to_increment(direction: &Direction) -> Position {
    match direction {
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
    }
}

pub fn count_perimeter_points(instructions: &Vec<DigInstruction>) -> usize {
    let result: isize = instructions
        .iter()
        .map(|instruction| instruction.steps)
        .sum();

    result as usize
}
