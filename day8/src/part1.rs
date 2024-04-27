use crate::{
    model::{Direction, Location},
    parse::{parse_instructions, parse_locations},
};
use std::collections::HashMap;

pub fn calculate_necessary_steps(text: &str) -> u32 {
    let components: Vec<&str> = text.split("\n\n").collect();

    let directions = parse_instructions(components.get(0).unwrap());
    let locations = parse_locations(components.get(1).unwrap());

    transverse_locations(locations, directions)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_necessary_steps() {
        let input_text = "RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)\nCCC = (ZZZ, GGG)\nDDD = (DDD, DDD)\nEEE = (EEE, EEE)\nGGG = (GGG, GGG)\nZZZ = (ZZZ, ZZZ)";

        let steps = calculate_necessary_steps(input_text);
        assert_eq!(steps, 2)
    }
}
