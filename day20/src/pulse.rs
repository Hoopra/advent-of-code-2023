#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PulseIntensity {
    Low,
    High,
}

#[derive(Debug, Clone)]
pub struct Pulse {
    pub intensity: PulseIntensity,
    pub origin: String,
    pub destination: String,
}

impl Pulse {
    pub fn default() -> Self {
        Self {
            intensity: PulseIntensity::Low,
            origin: String::from(""),
            destination: String::from("broadcaster"),
        }
    }

    pub fn new(intensity: PulseIntensity, origin: String, destination: String) -> Self {
        Self {
            intensity,
            origin,
            destination,
        }
    }
}
