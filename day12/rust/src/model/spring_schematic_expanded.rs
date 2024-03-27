use super::{SpringSchematic, SpringState};

const EXPANSIONS: usize = 5;

impl SpringSchematic {
    pub fn from_expanded_text(text: &str) -> SpringSchematic {
        let schematic = SpringSchematic::from_text(text);

        let mut states = schematic.states().to_vec();
        let mut groups = schematic.damaged_groups().to_vec();

        for _ in 1..EXPANSIONS {
            states.push(SpringState::Unknown);
            states.append(&mut schematic.states().to_vec());
            groups.append(&mut schematic.damaged_groups().to_vec());
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

        assert_eq!(schematic.states().len(), 7 * EXPANSIONS + EXPANSIONS - 1);
        assert_eq!(schematic.damaged_groups().len(), 3 * EXPANSIONS);
    }

    #[test]
    fn calculates_expanded_damaged_combinations() {
        let input = "????.#...#... 4,1,1";
        let schematic = SpringSchematic::from_expanded_text(input);

        let result = schematic.find_damaged_combinations();
        assert_eq!(result, 16);
    }
}
