use crate::brick::Brick;

pub fn move_bricks_to_rest(bricks: Vec<Brick>) -> Vec<Brick> {
    let mut resting = vec![];
    let mut not_resting: Vec<Brick> = vec![];

    for brick in bricks {
        if brick.is_resting_on_ground() {
            resting.push(brick);
            continue;
        }

        not_resting.push(brick);
    }

    not_resting.sort();
    not_resting.reverse();

    while not_resting.len() > 0 {
        let mut brick = not_resting.pop().unwrap();

        let z = brick.find_available_coordinate_below(&resting);
        brick.move_to_z(z);

        resting.push(brick);
    }

    resting
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::brick::{assign_brick_ids, parse_bricks, Position3D};

    #[test]
    fn moves_all_bricks_to_rest() {
        let input = "1,0,1~1,2,1\n0,0,2~2,0,2\n0,2,3~2,2,3\n0,0,4~0,2,4\n2,0,5~2,2,5\n0,1,6~2,1,6\n1,1,8~1,1,9";

        let bricks = parse_bricks(input);
        let bricks = assign_brick_ids(bricks);

        let result = move_bricks_to_rest(bricks);

        let expected: Vec<(Position3D, Position3D)> = vec![
            ((1, 0, 1), (1, 2, 1)),
            ((0, 0, 2), (2, 0, 2)),
            ((0, 2, 2), (2, 2, 2)),
        ];

        expected.iter().enumerate().for_each(|(i, (start, end))| {
            let brick = result.get(i).unwrap();

            assert_eq!(brick.start, *start);
            assert_eq!(brick.end, *end);
        });
    }
}
