use std::collections::HashMap;

use crate::{
    math::lowest_common_multiple,
    module::{Module, ModuleType},
    pulse::{Pulse, PulseIntensity},
};

use super::{
    construct_module_map, find_pulses_entries,
    model::{pulse_key, ModuleMap, PulseMap},
};

pub fn find_minimum_button_pushes_for_pulse(text: &str, target_pulse: &Pulse) -> u64 {
    let mut map = construct_module_map(text);

    let pulse_map = find_pulses_entries(&mut map, 10_000);

    let previous = find_module_inputs(&map, &target_pulse.destination);

    previous
        .keys()
        .map(|id| {
            let pulse = Pulse::new(
                target_pulse.intensity.clone(),
                id.to_string(),
                target_pulse.destination.clone(),
            );

            find_button_pushes_for_pulse(&map, &pulse_map, &pulse)
        })
        .min()
        .unwrap()
}

fn find_button_pushes_for_pulse(
    map: &ModuleMap,
    pulse_map: &PulseMap,
    target_pulse: &Pulse,
) -> u64 {
    let Pulse {
        intensity, origin, ..
    } = target_pulse;

    let module = map.get(origin).unwrap();

    let key = pulse_key(target_pulse);

    let default = vec![];
    let pushes = pulse_map.get(&key).unwrap_or(&default);

    let first_push = *pushes.first().unwrap_or(&0);

    if first_push > 0 {
        return first_push;
    }

    let previous = find_module_inputs(map, origin);

    match module.module_type {
        ModuleType::Untyped | ModuleType::Broadcaster => 1,
        ModuleType::Conjunction => {
            let new_intensity = match intensity {
                PulseIntensity::Low => PulseIntensity::High,
                _ => PulseIntensity::Low,
            };
            lowest_common_multiple(
                previous
                    .keys()
                    .map(|id| {
                        let pulse = Pulse::new(new_intensity, id.to_string(), origin.to_string());
                        find_button_pushes_for_pulse(map, pulse_map, &pulse)
                    })
                    .collect(),
            )
        }
        ModuleType::FlipFlop => 1,
    }
}

fn find_module_inputs<'a>(
    map: &'a ModuleMap,
    module_id: &String,
) -> HashMap<&'a String, &'a Module> {
    map.iter()
        .filter(|(_, module)| module.outputs.contains(module_id))
        .collect()
}
