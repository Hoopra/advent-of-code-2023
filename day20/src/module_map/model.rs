use std::collections::HashMap;

use crate::{
    module::Module,
    pulse::{Pulse, PulseIntensity},
};

pub type PulseMap = HashMap<(String, String, PulseIntensity), Vec<u64>>;

pub type ModuleMap = HashMap<String, Module>;

pub fn pulse_key(pulse: &Pulse) -> (String, String, PulseIntensity) {
    let Pulse {
        intensity,
        origin,
        destination,
    } = &pulse;

    (origin.to_string(), destination.to_string(), *intensity)
}
