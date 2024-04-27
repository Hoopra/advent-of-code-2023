use std::collections::HashMap;

use crate::model::{Direction, Location};

pub fn parse_instructions(text: &str) -> Vec<Direction> {
    text.chars()
        .map(|character| match character {
            'L' => Direction::Left,
            _ => Direction::Right,
        })
        .collect()
}

pub fn parse_locations(text: &str) -> HashMap<String, Location> {
    let mut location_map = HashMap::new();

    for line in text.lines() {
        let location = parse_location(line);
        location_map.insert(location.id.clone(), location);
    }

    location_map
}

fn parse_location(text: &str) -> Location {
    let components: Vec<&str> = text.split(" = ").collect();
    let id = components.get(0).unwrap();

    let (left_id, right_id) = parse_ids(components.get(1).unwrap());

    Location::new(String::from(*id), left_id, right_id)
}

fn parse_ids(input: &str) -> (String, String) {
    let replaced = input.replace('(', "");
    let replaced = replaced.replace(')', "");
    let replaced = replaced.replace(',', "");
    let replaced = replaced.replace(' ', "");

    let (left_id, right_id) = replaced.split_at(3);

    (String::from(left_id), String::from(right_id))
}
