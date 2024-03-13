mod model;

use model::{Direction, Location};
use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let file_path = "../input.txt";
    let text = read_to_string(file_path).unwrap();

    let steps_taken = calculate_necessary_steps(&text, false);
    assert_eq!(steps_taken, 23147);

    // let ghost_steps_taken = calculate_necessary_steps(&text, true);
    // assert_eq!(ghost_steps_taken, 23147);
}

fn calculate_necessary_steps(text: &str, is_ghost: bool) -> u32 {
    let components: Vec<&str> = text.split("\n\n").collect();

    let directions = parse_instructions(components.get(0).unwrap());
    let locations = parse_locations(components.get(1).unwrap());

    match is_ghost {
        false => transverse_locations(locations, directions),
        true => transverse_ghost_locations(locations, directions),
    }
}

fn transverse_locations(locations: HashMap<String, Location>, directions: Vec<Direction>) -> u32 {
    let mut num_steps = 0;
    let mut current = 0;
    let mut current_id = "AAA";

    while current_id != "ZZZ" {
        let current_location = locations.get(current_id).unwrap();
        let direction = directions.get(current).unwrap();

        current_id = match direction {
            Direction::Left => &current_location.left_id,
            Direction::Right => &current_location.right_id,
        };

        num_steps += 1;
        current += 1;
        if current >= directions.len() {
            current = 0;
        }
    }

    num_steps
}

fn transverse_ghost_locations(
    locations: HashMap<String, Location>,
    directions: Vec<Direction>,
) -> u32 {
    let mut num_steps = 0;
    let mut current = 0;

    let mut current_ids: Vec<String> = locations
        .keys()
        .filter(|key| key.chars().last().unwrap() == 'A')
        .map(|key| String::from(key))
        .collect();

    let mut matches = 0;

    while matches < current_ids.len() {
        let direction = directions.get(current).unwrap();

        current_ids = current_ids
            .iter()
            .map(|id| match direction {
                Direction::Left => locations.get(id).unwrap().left_id.to_owned(),
                Direction::Right => locations.get(id).unwrap().right_id.to_owned(),
            })
            .collect();

        num_steps += 1;
        current += 1;

        matches = current_ids
            .iter()
            .filter(|id| id.chars().last().unwrap() == 'Z')
            .collect::<Vec<&String>>()
            .len();

        if current >= directions.len() {
            current = 0;
        }
    }

    num_steps
}

fn parse_instructions(text: &str) -> Vec<Direction> {
    text.chars()
        .map(|character| match character {
            'L' => Direction::Left,
            _ => Direction::Right,
        })
        .collect()
}

fn parse_locations(text: &str) -> HashMap<String, Location> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_necessary_steps() {
        let input_text = "RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)\nCCC = (ZZZ, GGG)\nDDD = (DDD, DDD)\nEEE = (EEE, EEE)\nGGG = (GGG, GGG)\nZZZ = (ZZZ, ZZZ)";

        let steps = calculate_necessary_steps(input_text, false);
        assert_eq!(steps, 2)
    }

    #[test]
    fn calculates_necessary_ghost_steps() {
        let input_text = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)";

        let steps = calculate_necessary_steps(input_text, true);
        assert_eq!(steps, 6)
    }
}
