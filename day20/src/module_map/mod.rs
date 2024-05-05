mod create;
mod model;
mod process;
mod target_pulse;

pub use create::construct_module_map;
pub use process::{find_pulses_entries, process_module_pulses};
pub use target_pulse::find_minimum_button_pushes_for_pulse;
