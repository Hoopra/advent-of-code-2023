use crate::pulse::{Pulse, PulseIntensity};

pub trait PulseHandler {
    fn receive_pulse(&mut self, pulse: &Pulse) -> Option<PulseIntensity>;
}

#[derive(Debug, Clone)]
pub enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcaster,
    Untyped,
}
