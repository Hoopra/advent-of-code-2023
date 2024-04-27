#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

pub type SpringStates = Vec<SpringState>;
