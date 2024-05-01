use std::{collections::HashMap, fs::read_to_string};

use crate::pulse::PulseIntensity;
use module::{Module, ModuleType, PulseHandler};
use pulse::Pulse;

mod module;
mod pulse;

fn main() {
    solve_part_1("input.txt");
}

fn solve_part_1(path_name: &str) {
    let text = read_to_string(path_name).unwrap();

    let (low, high) = process_module_pulses(&text, Some(1000));
    let result = low * high;

    assert_eq!(result, 898731036);
}

fn construct_module_map(text: &str) -> HashMap<String, Module> {
    let mut inputs: HashMap<String, Vec<String>> = HashMap::new();

    let mut modules: HashMap<String, Module> = text
        .lines()
        .into_iter()
        .map(|line| {
            let components: Vec<&str> = line.split_whitespace().collect();

            let name = *components.first().unwrap();

            let id = match name {
                "broadcaster" => String::from(name),
                _ => name.chars().skip(1).collect(),
            };

            let outputs: Vec<String> = components
                .iter()
                .skip(2)
                .map(|value| value.chars().filter(|symbol| symbol != &',').collect())
                .collect();

            for output in &outputs {
                let mut target = inputs.get(output).unwrap_or(&vec![]).to_vec();
                target.push(id.to_string());

                inputs.insert(output.to_string(), target);
            }

            match name {
                _ if name.contains("&") => (id, Module::new(ModuleType::Conjunction, outputs)),
                _ if name.contains("%") => (id, Module::new(ModuleType::FlipFlop, outputs)),
                "broadcaster" => (id, Module::new(ModuleType::Broadcaster, outputs)),
                _ => (id, Module::new(ModuleType::Untyped, outputs)),
            }
        })
        .collect();

    for (id, module) in modules.iter_mut() {
        let input_modules = inputs.get(id);

        if input_modules.is_none() {
            continue;
        }

        module.set_inputs(input_modules.unwrap().to_vec());
    }

    modules
}

fn process_module_pulses(text: &str, times: Option<u32>) -> (u32, u32) {
    let mut map = construct_module_map(text);

    println!("map.len: {:?}", map.len());
    println!("map.len: {:?}", map);

    let mut low_pulses = 0;
    let mut high_pulses = 0;
    let times = times.unwrap_or(1);

    for _ in 0..times {
        let mut pulses = vec![Pulse::default()];

        while pulses.len() > 0 {
            let (low, high) = process_pulse_cycle(&mut map, &mut pulses);

            low_pulses += low;
            high_pulses += high;
        }
    }

    (low_pulses, high_pulses)
}

fn process_pulse_cycle(map: &mut HashMap<String, Module>, pulses: &mut Vec<Pulse>) -> (u32, u32) {
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

    (low_pulses, high_pulses)
}

fn handle_pulse(map: &mut HashMap<String, Module>, pulse: &Pulse) -> Vec<Pulse> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_module_map() {
        let text = "broadcaster -> a, b, c\n%a -> b\n%b -> c\n%c -> inv\n&inv -> a";

        let result = construct_module_map(text);

        assert_eq!(result.len(), 5);
    }

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
