use crate::brick::Brick;
use std::collections::HashMap;

pub struct BrickRelationships {
    pub directly_below: HashMap<String, Vec<String>>,
    pub directly_above: HashMap<String, Vec<String>>,
}

pub fn find_supported_bricks(bricks: &Vec<Brick>) -> BrickRelationships {
    let mut directly_below = HashMap::new();
    let mut directly_above = HashMap::new();

    bricks.iter().for_each(|brick| {
        let overlapping = brick.find_overlapping(bricks);

        let mut above_brick: Vec<String> = vec![];
        let mut below_brick: Vec<String> = vec![];

        for overlapping_brick in overlapping {
            let id = overlapping_brick.id.to_string();

            if overlapping_brick.is_on_top_of(brick) {
                above_brick.push(id);
            } else if brick.is_on_top_of(overlapping_brick) {
                below_brick.push(id);
            }
        }

        let id = brick.id.to_string();

        directly_above.insert(id.clone(), above_brick);
        directly_below.insert(id, below_brick);
    });

    BrickRelationships {
        directly_above,
        directly_below,
    }
}
