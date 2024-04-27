use super::{SpringSchematic, SpringState};

const EXPANSIONS: usize = 5;

impl SpringSchematic {
    pub fn from_expanded_text(text: &str) -> SpringSchematic {
        let schematic = SpringSchematic::from_text(text);

        let mut states = schematic.get_states().to_vec();
        let mut groups = schematic.get_damaged_groups().to_vec();

        for _ in 1..EXPANSIONS {
            states.push(SpringState::Unknown);
            states.append(&mut schematic.get_states().to_vec());
            groups.append(&mut schematic.get_damaged_groups().to_vec());
        }

        SpringSchematic::new(states, groups)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_expanded_spring_schematic() {
        let input = "#.#.##? 1,1,3";

        let schematic = SpringSchematic::from_expanded_text(input);

        assert_eq!(schematic.get_states().len(), (8 * EXPANSIONS) - 1);
        assert_eq!(schematic.get_damaged_groups().len(), 3 * EXPANSIONS);
    }
}
