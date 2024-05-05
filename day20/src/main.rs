mod math;
mod module;
mod module_map;
mod pulse;

use module_map::process_module_pulses;
use std::fs::read_to_string;

use crate::{module_map::find_minimum_button_pushes_for_pulse, pulse::Pulse};

fn main() {
    solve_part_1("input.txt");
    solve_part_2("input.txt");
}

fn solve_part_1(path_name: &str) {
    let text = read_to_string(path_name).unwrap();

    let (low, high) = process_module_pulses(&text, Some(1000));
    let result = low * high;

    assert_eq!(result, 898731036);
}

fn solve_part_2(path_name: &str) {
    let text = read_to_string(path_name).unwrap();

    let target_pulse = Pulse::new(
        pulse::PulseIntensity::Low,
        String::new(),
        String::from("rx"),
    );

    let times = find_minimum_button_pushes_for_pulse(&text, &target_pulse);

    assert_eq!(times, 229414480926893);
}
