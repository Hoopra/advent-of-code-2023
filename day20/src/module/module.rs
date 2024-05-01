use std::collections::HashMap;

use crate::pulse::{Pulse, PulseIntensity};

use super::{model::ModuleType, PulseHandler};

#[derive(Debug, Clone)]
pub struct Module {
    module_type: ModuleType,
    pub outputs: Vec<String>,

    // flip flop
    turned_on: bool,

    // conjunction
    previous_inputs: HashMap<String, PulseIntensity>,
}

impl Module {
    pub fn new(module_type: ModuleType, outputs: Vec<String>) -> Self {
        Self {
            outputs,
            module_type,
            turned_on: false,
            previous_inputs: HashMap::new(),
        }
    }
}

impl Module {
    fn receive_pulse_flip_flop(&mut self, pulse: &PulseIntensity) -> Option<PulseIntensity> {
        match pulse {
            PulseIntensity::Low => {
                self.turned_on = !self.turned_on;

                match self.turned_on {
                    true => Some(PulseIntensity::High),
                    false => Some(PulseIntensity::Low),
                }
            }
            PulseIntensity::High => None,
        }
    }

    fn receive_pulse_conjunction(&mut self, pulse: &Pulse) -> Option<PulseIntensity> {
        let Pulse {
            intensity, origin, ..
        } = pulse;

        self.previous_inputs.insert(origin.to_string(), *intensity);

        match self
            .previous_inputs
            .values()
            .all(|value| value == &PulseIntensity::High)
        {
            true => Some(PulseIntensity::Low),
            _ => Some(PulseIntensity::High),
        }
    }

    fn receive_pulse_broadcaster(&mut self, pulse: &PulseIntensity) -> Option<PulseIntensity> {
        Some(*pulse)
    }

    fn receive_pulse_untyped(&mut self, _pulse: &PulseIntensity) -> Option<PulseIntensity> {
        None
    }

    pub fn set_inputs(&mut self, inputs: Vec<String>) {
        self.previous_inputs.clear();

        for origin in &inputs {
            self.previous_inputs
                .insert(origin.to_string(), PulseIntensity::Low);
        }
    }
}

impl PulseHandler for Module {
    fn receive_pulse(&mut self, pulse: &Pulse) -> Option<PulseIntensity> {
        let intensity = pulse.intensity;

        match self.module_type {
            ModuleType::FlipFlop => self.receive_pulse_flip_flop(&intensity),
            ModuleType::Conjunction => self.receive_pulse_conjunction(pulse),
            ModuleType::Broadcaster => self.receive_pulse_broadcaster(&intensity),
            _ => self.receive_pulse_untyped(&intensity),
        }
    }
}
