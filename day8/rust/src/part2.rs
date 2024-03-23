use num::integer::lcm;
use std::collections::HashMap;

use crate::{
    model::{Direction, Location},
    parse::{parse_instructions, parse_locations},
};

pub fn calculate_necessary_ghost_steps(text: &str) -> u128 {
    let components: Vec<&str> = text.split("\n\n").collect();

    let directions = parse_instructions(components.get(0).unwrap());
    let locations = parse_locations(components.get(1).unwrap());

    let starting_locations = find_locations_ids_ending_with(&locations, 'A');

    let loop_lengths: Vec<u128> = starting_locations
        .iter()
        .map(|id| find_loop_length_for_id(id, &locations, &directions))
        .collect();

    lowest_common_multiple(loop_lengths)
}

fn find_locations_ids_ending_with(
    locations: &HashMap<String, Location>,
    end_character: char,
) -> Vec<String> {
    locations
        .keys()
        .filter_map(|id| match id.ends_with(end_character) {
            false => None,
            true => Some(id.to_owned()),
        })
        .collect()
}

fn find_loop_length_for_id(
    id: &str,
    locations: &HashMap<String, Location>,
    directions: &Vec<Direction>,
) -> u128 {
    let mut num_steps = 0;
    let mut direction_index = 0;
    let mut current_id = id;

    loop {
        let current_location = locations.get(current_id).unwrap();
        let direction = directions.get(direction_index).unwrap();

        if current_id.ends_with('Z') {
            break num_steps;
        }

        current_id = match direction {
            Direction::Left => &current_location.left_id,
            Direction::Right => &current_location.right_id,
        };

        num_steps += 1;
        direction_index += 1;

        if direction_index >= directions.len() {
            direction_index = 0;
        }
    }
}

fn lowest_common_multiple(numbers: Vec<u128>) -> u128 {
    numbers
        .iter()
        .skip(1)
        .fold(*numbers.get(0).unwrap(), |previous, next| {
            lcm::<u128>(previous, *next)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_lowest_common_denominator() {
        let numbers = vec![12, 4, 22];

        assert_eq!(lowest_common_multiple(numbers), 132)
    }

    #[test]
    fn calculates_ghost_steps() {
        let input = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)";

        assert_eq!(calculate_necessary_ghost_steps(input), 6)
    }
}
