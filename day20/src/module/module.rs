use std::collections::HashMap;

use crate::pulse::{Pulse, PulseIntensity};

use super::{model::ModuleType, PulseHandler};

#[derive(Debug, Clone)]
pub struct Module {
    pub module_type: ModuleType,
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
            }
            PulseIntensity::High => {}
        }

        self.send_current_pulse(pulse)
    }

    fn receive_pulse_conjunction(&mut self, pulse: &Pulse) -> Option<PulseIntensity> {
        let Pulse {
            intensity, origin, ..
        } = pulse;

        self.previous_inputs.insert(origin.to_string(), *intensity);

        self.send_current_pulse(intensity)
    }

    pub fn set_inputs(&mut self, inputs: Vec<String>) {
        self.previous_inputs.clear();

        for origin in &inputs {
            self.previous_inputs
                .insert(origin.to_string(), PulseIntensity::Low);
        }
    }

    fn all_inputs_high(&self) -> bool {
        self.previous_inputs
            .values()
            .all(|value| value == &PulseIntensity::High)
    }

    pub fn send_current_pulse(&self, input_pulse: &PulseIntensity) -> Option<PulseIntensity> {
        match self.module_type {
            ModuleType::FlipFlop => match input_pulse {
                PulseIntensity::Low => match self.turned_on {
                    true => Some(PulseIntensity::High),
                    false => Some(PulseIntensity::Low),
                },
                PulseIntensity::High => None,
            },
            ModuleType::Broadcaster => Some(*input_pulse),
            ModuleType::Conjunction => match self.all_inputs_high() {
                true => Some(PulseIntensity::Low),
                _ => Some(PulseIntensity::High),
            },
            ModuleType::Untyped => None,
        }
    }
}

impl PulseHandler for Module {
    fn receive_pulse(&mut self, pulse: &Pulse) -> Option<PulseIntensity> {
        let intensity = pulse.intensity;

        match self.module_type {
            ModuleType::FlipFlop => self.receive_pulse_flip_flop(&intensity),
            ModuleType::Conjunction => self.receive_pulse_conjunction(pulse),
            ModuleType::Broadcaster => self.send_current_pulse(&pulse.intensity),
            _ => self.send_current_pulse(&pulse.intensity),
        }
    }
}
