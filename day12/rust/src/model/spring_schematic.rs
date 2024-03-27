use super::{find_possible_state_arrangements, spring_states_to_groups, SpringState, SpringStates};

#[derive(Debug)]
pub struct SpringSchematic {
    states: SpringStates,
    damaged_groups: Vec<usize>,
}

impl SpringSchematic {
    pub fn new(states: SpringStates, damaged_groups: Vec<usize>) -> SpringSchematic {
        SpringSchematic {
            states,
            damaged_groups,
        }
    }

    pub fn from_text(text: &str) -> SpringSchematic {
        let components: Vec<&str> = text.split_whitespace().collect();

        let states = components.get(0).unwrap();
        let groups = components.get(1).unwrap();

        SpringSchematic {
            states: states
                .chars()
                .map(|symbol| match symbol {
                    '.' => SpringState::Operational,
                    '#' => SpringState::Damaged,
                    _ => SpringState::Unknown,
                })
                .collect(),

            damaged_groups: groups
                .split(',')
                .map(|symbol| symbol.to_string().parse().unwrap())
                .collect(),
        }
    }
}

impl SpringSchematic {
    pub fn states(&self) -> &SpringStates {
        &self.states
    }
    pub fn damaged_groups(&self) -> &Vec<usize> {
        &self.damaged_groups
    }

    pub fn find_damaged_combinations(&self) -> usize {
        let states = &self.states;
        let permutations = find_possible_state_arrangements(states);

        if permutations.len() == 0 {
            return 1;
        }

        permutations
            .iter()
            .filter(|state| is_arrangement_valid(state, &self.damaged_groups))
            .count()
    }
}

fn is_arrangement_valid(states: &SpringStates, damaged_groups: &Vec<usize>) -> bool {
    let state_groups = spring_states_to_groups(states);

    if state_groups.len() != damaged_groups.len() {
        return false;
    }

    state_groups.iter().zip(damaged_groups).all(|(a, b)| a == b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_spring_schematic() {
        let input = "#.#.##? 1,1,3";

        let schematic = SpringSchematic::from_text(input);

        assert_eq!(
            schematic.states,
            vec![
                SpringState::Damaged,
                SpringState::Operational,
                SpringState::Damaged,
                SpringState::Operational,
                SpringState::Damaged,
                SpringState::Damaged,
                SpringState::Unknown,
            ]
        );
        assert_eq!(schematic.damaged_groups, vec![1, 1, 3]);
    }

    #[test]
    fn calculates_damaged_combinations() {
        let input = "#.#.### 1,1,3";
        let schematic = SpringSchematic::from_text(input);

        let result = schematic.find_damaged_combinations();
        assert_eq!(result, 1);

        let input = ".??..??...?##. 1,1,3";
        let schematic = SpringSchematic::from_text(input);

        let result = schematic.find_damaged_combinations();
        assert_eq!(result, 4);

        let input = "????.######..#####. 1,6,5";
        let schematic = SpringSchematic::from_text(input);

        let result = schematic.find_damaged_combinations();
        assert_eq!(result, 4);

        let input = "?###???????? 3,2,1";
        let schematic = SpringSchematic::from_text(input);

        let result = schematic.find_damaged_combinations();
        assert_eq!(result, 10);
    }

    #[test]
    fn converts_spring_states_to_groups() {
        let input = "..#..# 1,1";
        let schematic = SpringSchematic::from_text(input);

        let result = spring_states_to_groups(&schematic.states);
        assert_eq!(result, vec![1, 1])
    }
}
