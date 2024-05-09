use std::cmp::{max, min};

pub type Position3D = (isize, isize, isize);

#[derive(Debug, PartialEq, Eq)]
pub struct Brick {
    pub id: String,
    pub start: Position3D,
    pub end: Position3D,
}

impl Brick {
    pub fn from_text(text: &str) -> Self {
        let components: Vec<&str> = text.split('~').collect();

        Brick {
            id: text.to_string(),
            start: position_3d_from_text(components.get(0).unwrap()),
            end: position_3d_from_text(components.get(1).unwrap()),
        }
    }
}

impl Brick {
    pub fn is_resting_on_ground(&self) -> bool {
        self.start.2 == 1 || self.end.2 == 1
    }

    pub fn move_down_one(&mut self) {
        self.start.2 -= 1;
        self.end.2 -= 1;
    }

    pub fn is_resting_on_other(&self, other: &Vec<Brick>) -> bool {
        self.find_overlapping(other)
            .iter()
            .any(|other| self.is_on_top_of(other))
    }

    pub fn is_on_top_of(&self, other: &Brick) -> bool {
        let least_self = min(self.start.2, self.end.2);
        let greatest_other = max(other.start.2, other.end.2);

        least_self - greatest_other == 1
    }

    pub fn find_overlapping<'a>(&self, other: &'a Vec<Brick>) -> Vec<&'a Brick> {
        other
            .iter()
            .filter(|other| self.overlaps_with(other))
            .collect()
    }

    fn overlaps_with(&self, other: &Brick) -> bool {
        let overlap_x = self
            .x_positions()
            .iter()
            .any(|value| other.x_positions().contains(value));

        let overlap_y = self
            .y_positions()
            .iter()
            .any(|value| other.y_positions().contains(value));

        overlap_x && overlap_y
    }

    fn x_positions(&self) -> Vec<isize> {
        find_position_range(self.start.0, self.end.0)
    }

    fn y_positions(&self) -> Vec<isize> {
        find_position_range(self.start.1, self.end.1)
    }
}

fn find_position_range(start: isize, end: isize) -> Vec<isize> {
    let mut result = vec![];

    for i in min(start, end)..=max(start, end) {
        result.push(i);
    }

    result
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let least_self = min(self.start.2, self.end.2);
        let least_other = min(other.start.2, other.end.2);

        least_self.partial_cmp(&least_other)
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let least_self = min(self.start.2, self.end.2);
        let least_other = min(other.start.2, other.end.2);

        least_self.cmp(&least_other)
    }
}

fn position_3d_from_text(text: &str) -> Position3D {
    let components: Vec<&str> = text.split(',').collect();

    (
        string_to_coordinate(components.get(0).unwrap()),
        string_to_coordinate(components.get(1).unwrap()),
        string_to_coordinate(components.get(2).unwrap()),
    )
}

fn string_to_coordinate(text: &str) -> isize {
    text.to_string().parse().unwrap()
}

pub fn parse_bricks(text: &str) -> Vec<Brick> {
    text.lines().map(|line| Brick::from_text(line)).collect()
}

#[cfg(test)]
pub fn assign_brick_ids(bricks: Vec<Brick>) -> Vec<Brick> {
    let ids = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];

    bricks
        .into_iter()
        .enumerate()
        .map(|(index, mut brick)| {
            brick.id = ids.get(index).unwrap().to_string().to_uppercase();
            brick
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_brick_from_text() {
        let input = "1,0,1~1,2,1";

        let brick = Brick::from_text(input);

        assert_eq!(brick.start, (1, 0, 1));
        assert_eq!(brick.end, (1, 2, 1));
    }

    #[test]
    fn parses_bricks_from_text() {
        let input = "1,0,1~1,2,1\n0,0,2~2,0,2\n0,2,3~2,2,3\n0,0,4~0,2,4\n2,0,5~2,2,5\n0,1,6~2,1,6\n1,1,8~1,1,9";

        let result = parse_bricks(input);
        assert_eq!(result.len(), 7);

        let brick = result.get(6).unwrap();

        assert_eq!(brick.start, (1, 1, 8));
        assert_eq!(brick.end, (1, 1, 9));
    }
}
