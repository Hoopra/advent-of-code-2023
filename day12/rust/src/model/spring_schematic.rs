use crate::permutation::create_permutations;

#[derive(Debug, PartialEq, Clone, Copy)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

type SpringStates = Vec<SpringState>;

#[derive(Debug)]
pub struct SpringSchematic {
    states: SpringStates,
    damaged_groups: Vec<usize>,
}

impl SpringSchematic {
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
    pub fn find_damaged_combinations(&self) -> usize {
        let states = &self.states;
        let permutations = find_group_permutations(states);

        if permutations.len() == 0 {
            return 1;
        }

        permutations
            .iter()
            .filter(|state| is_permutation_possible(state, &self.damaged_groups))
            .count()
    }
}

fn find_group_permutations(states: &SpringStates) -> Vec<SpringStates> {
    let unknowns = states
        .iter()
        .filter(|state| state == &&SpringState::Unknown)
        .count();

    let unknown_state_permutations = create_permutations(unknowns);

    unknown_state_permutations
        .iter()
        .map(|unknown_state| create_permutation(states, unknown_state))
        .collect()
}

fn create_permutation(states: &SpringStates, unknown_states: &Vec<bool>) -> SpringStates {
    let mut replace_index = 0;

    states
        .iter()
        .map(|state| match state {
            SpringState::Unknown => {
                let unknown_state = unknown_states.get(replace_index).unwrap();
                replace_index += 1;

                match unknown_state {
                    true => SpringState::Operational,
                    false => SpringState::Damaged,
                }
            }
            _ => *state,
        })
        .collect()
}

fn is_permutation_possible(states: &SpringStates, damaged_groups: &Vec<usize>) -> bool {
    let state_groups = spring_states_to_groups(states);

    if state_groups.len() != damaged_groups.len() {
        return false;
    }

    state_groups.iter().zip(damaged_groups).all(|(a, b)| a == b)
}

fn spring_states_to_groups(states: &SpringStates) -> Vec<usize> {
    let mut groups: Vec<usize> = vec![];
    let mut group_size = 0;

    for value in states {
        match value {
            SpringState::Operational => {
                if group_size > 0 {
                    groups.push(group_size);
                }

                group_size = 0;
            }
            _ => group_size += 1,
        }
    }

    if group_size > 0 {
        groups.push(group_size)
    }

    groups
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

    #[test]
    fn finds_group_permutations() {
        let input = "..?..? 1,1";
        let schematic = SpringSchematic::from_text(input);

        let result = find_group_permutations(&schematic.states);
        assert_eq!(result.len(), 4);

        let input = ".??.?? 1,1";
        let schematic = SpringSchematic::from_text(input);

        let result = find_group_permutations(&schematic.states);
        assert_eq!(result.len(), 16);

        let input = "?????? 1,1";
        let schematic = SpringSchematic::from_text(input);

        let result = find_group_permutations(&schematic.states);
        assert_eq!(result.len(), 64);
    }
}
