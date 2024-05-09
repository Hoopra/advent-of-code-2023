mod brick;
mod brick_location;
mod brick_movement;

use brick::{parse_bricks, Brick};
use brick_location::{find_supported_bricks, BrickRelationships};
use brick_movement::move_bricks_to_rest;
use std::fs::read_to_string;

fn main() {
    solve_part_1();
}

fn solve_part_1() {
    let text = read_to_string("input.txt").unwrap();

    let bricks = parse_bricks(&text);
    let result = find_removable_bricks(bricks);
    assert_eq!(result.len(), 448);
}

fn find_removable_bricks(bricks: Vec<Brick>) -> Vec<String> {
    let bricks = move_bricks_to_rest(bricks);

    let BrickRelationships {
        directly_above,
        directly_below,
    } = find_supported_bricks(&bricks);

    let can_be_removed: Vec<String> = bricks
        .iter()
        .filter_map(|brick| {
            let id = &brick.id;
            let above_brick = directly_above.get(id).unwrap();

            if above_brick.len() == 0 {
                return Some(id.to_string());
            }

            let supported_has_multiple_supports = above_brick.iter().all(|above_id| {
                let below = directly_below.get(above_id).unwrap();

                below.len() > 1
            });

            match supported_has_multiple_supports {
                true => Some(id.to_string()),
                _ => None,
            }
        })
        .collect();

    can_be_removed
}

#[cfg(test)]
mod tests {
    use crate::brick::assign_brick_ids;

    use super::*;

    #[test]
    fn finds_removable_bricks() {
        let input = "1,0,1~1,2,1\n0,0,2~2,0,2\n0,2,3~2,2,3\n0,0,4~0,2,4\n2,0,5~2,2,5\n0,1,6~2,1,6\n1,1,8~1,1,9";

        let bricks = parse_bricks(input);
        let bricks = assign_brick_ids(bricks);

        let result = find_removable_bricks(bricks);

        assert_eq!(result.len(), 5);

        let expected = vec!["B", "C", "D", "E", "G"];

        for id in result {
            assert!(expected.contains(&id.as_ref()))
        }
    }
}
