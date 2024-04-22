use std::collections::HashMap;

pub type Position = (isize, isize);

pub struct PerimeterNode {
    pub position: Position,
    pub symbol: char,
}

impl PerimeterNode {
    pub fn new(position: Position, symbol: char) -> PerimeterNode {
        PerimeterNode { position, symbol }
    }
}

pub type EnclosingArea = (Position, Position);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// returns bounding area as top left corner, bottom right corner
pub fn find_bounding_area(perimeter: &HashMap<Position, PerimeterNode>) -> EnclosingArea {
    let mut top_left = (0, 0);
    let mut bottom_right = (0, 0);

    for position in perimeter.keys() {
        let (x, y) = *position;

        if x < top_left.0 {
            top_left.0 = x;
        }

        if x > bottom_right.0 {
            bottom_right.0 = x;
        }

        if y < top_left.1 {
            top_left.1 = y;
        }

        if y > bottom_right.1 {
            bottom_right.1 = y;
        }
    }

    (top_left, bottom_right)
}

pub fn find_enclosed_area(
    area: &EnclosingArea,
    perimeter: &HashMap<Position, PerimeterNode>,
) -> Vec<Position> {
    let (top_left, bottom_right) = area;
    let (x_min, y_min) = *top_left;
    let (x_max, y_max) = *bottom_right;

    let mut result = vec![];

    for x in x_min..=x_max {
        for y in y_min..=y_max {
            let position = (x, y);

            let is_on_perimeter = perimeter.get(&position).is_some();

            if is_on_perimeter || is_position_enclosed(&position, perimeter) {
                result.push(position)
            }
        }
    }

    result
}

fn is_position_enclosed(position: &Position, perimeter: &HashMap<Position, PerimeterNode>) -> bool {
    is_position_enclosed_right(position, perimeter)
}

fn is_position_enclosed_right(
    position: &Position,
    perimeter: &HashMap<Position, PerimeterNode>,
) -> bool {
    let perimeter: Vec<char> = find_perimeter_nodes_right(position, perimeter);

    is_position_enclosed_by_pipes(perimeter, '-')
}

fn find_perimeter_nodes_right(
    base_position: &Position,
    perimeter: &HashMap<Position, PerimeterNode>,
) -> Vec<char> {
    perimeter
        .values()
        .filter_map(|node| {
            let PerimeterNode { position, symbol } = node;

            let (x, y) = position;
            let (x_base, y_base) = base_position;

            match y == y_base && x > x_base {
                true => Some(*symbol),
                false => None,
            }
        })
        .collect()
}

fn is_position_enclosed_by_pipes(perimeter: Vec<char>, excluded_symbol: char) -> bool {
    let mut vertical: f32 = 0.0;
    let mut bends_1: f32 = 0.0;
    let mut bends_2: f32 = 0.0;

    perimeter.iter().for_each(|symbol| match *symbol {
        'L' | '7' => bends_1 += 0.5,
        'F' | 'J' => bends_2 += 0.5,
        symbol if symbol == excluded_symbol => {}
        _ => vertical += 1.0,
    });

    let score = vertical + bends_1 - bends_2;

    score % 2.0 != 0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_bounding_area() {
        let perimeter = HashMap::from([
            ((-2, -2), PerimeterNode::new((-2, -2), '|')),
            ((0, 0), PerimeterNode::new((0, 0), '7')),
            ((3, 5), PerimeterNode::new((3, 5), '-')),
            ((5, 3), PerimeterNode::new((5, 3), '7')),
        ]);

        let (top_left, bottom_right) = find_bounding_area(&perimeter);

        assert_eq!(top_left, (-2, -2));
        assert_eq!(bottom_right, (5, 5));
    }
}
