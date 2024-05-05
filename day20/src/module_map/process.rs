use super::construct_module_map;
use super::model::{pulse_key, ModuleMap, PulseMap};
use crate::module::PulseHandler;
use crate::pulse::{Pulse, PulseIntensity};
use std::collections::HashMap;

pub fn process_module_pulses(text: &str, times: Option<u32>) -> (u32, u32) {
    let mut map = construct_module_map(text);

    let mut low_pulses = 0;
    let mut high_pulses = 0;
    let times = times.unwrap_or(1);

    for _ in 0..times {
        let mut pulses = vec![Pulse::default()];

        while pulses.len() > 0 {
            let (low, high, _) = process_pulse_cycle(&mut map, &mut pulses);

            low_pulses += low;
            high_pulses += high;
        }
    }

    (low_pulses, high_pulses)
}

fn process_pulse_cycle(map: &mut ModuleMap, pulses: &mut Vec<Pulse>) -> (u32, u32, bool) {
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    let current_pulses = pulses.to_vec();
    pulses.clear();

    for pulse in current_pulses {
        match &pulse.intensity {
            PulseIntensity::Low => low_pulses += 1,
            PulseIntensity::High => high_pulses += 1,
        }

        let mut new_pulses = handle_pulse(map, &pulse);
        pulses.append(&mut new_pulses);
    }

    (low_pulses, high_pulses, false)
}

fn handle_pulse(map: &mut ModuleMap, pulse: &Pulse) -> Vec<Pulse> {
    let Pulse { destination, .. } = pulse;

    let module = map.get(destination);

    if module.is_none() {
        return vec![];
    }

    let mut new_module = module.unwrap().clone();

    let pulse = new_module.receive_pulse(&pulse);
    let outputs = new_module.outputs.to_vec();

    map.insert(destination.to_string(), new_module);

    if pulse.is_none() {
        return vec![];
    }

    outputs
        .iter()
        .map(|new_destination| {
            Pulse::new(
                pulse.unwrap(),
                destination.to_string(),
                new_destination.to_string(),
            )
        })
        .collect()
}

pub fn find_pulses_entries(map: &mut ModuleMap, times: u64) -> PulseMap {
    let mut pulse_map = HashMap::new();

    for i in 1..=times {
        let mut pulses = vec![Pulse::default()];

        while pulses.len() > 0 {
            find_pulses_in_cycle(map, &mut pulses, &mut pulse_map, i);
        }
    }

    pulse_map
}

fn find_pulses_in_cycle(
    map: &mut ModuleMap,
    pulses: &mut Vec<Pulse>,
    pulse_map: &mut PulseMap,
    iteration: u64,
) {
    let current_pulses = pulses.to_vec();
    pulses.clear();

    for pulse in current_pulses {
        let key = pulse_key(&pulse);

        let entry = pulse_map.get_mut(&key);
        match entry {
            None => {
                pulse_map.insert(key, vec![iteration]);
            }
            Some(pushes) => pushes.push(iteration),
        }

        let mut new_pulses = handle_pulse(map, &pulse);
        pulses.append(&mut new_pulses);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn processes_one_pulse_step() {
        let text = "broadcaster -> a, b, c\n%a -> b\n%b -> c\n%c -> inv\n&inv -> a";

        let result = process_module_pulses(text, None);

        assert_eq!(result, (8, 4));
    }

    #[test]
    fn processes_one_pulse_step_other_example() {
        let text = "broadcaster -> a\n%a -> inv, con\n&inv -> b\n%b -> con\n&con -> output";

        let result = process_module_pulses(text, None);

        assert_eq!(result, (4, 4));
    }

    #[test]
    fn processes_1000_pulse_step() {
        let text = "broadcaster -> a\n%a -> inv, con\n&inv -> b\n%b -> con\n&con -> output";

        let (low, high) = process_module_pulses(text, Some(1000));

        assert_eq!(low, 4250);
        assert_eq!(high, 2750);

        assert_eq!(low * high, 11687500);
    }
}
