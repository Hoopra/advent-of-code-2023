use crate::permutation::create_permutations;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

pub type SpringStates = Vec<SpringState>;

pub fn spring_states_to_groups(states: &SpringStates) -> Vec<usize> {
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

pub fn find_possible_state_arrangements(states: &SpringStates) -> Vec<SpringStates> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_spring_states_to_groups() {
        let states = vec![
            SpringState::Operational,
            SpringState::Operational,
            SpringState::Damaged,
            SpringState::Operational,
            SpringState::Operational,
            SpringState::Damaged,
        ];

        let result = spring_states_to_groups(&states);
        assert_eq!(result, vec![1, 1])
    }

    #[test]
    fn finds_possible_arrangements_of_group() {
        let states = vec![
            SpringState::Operational,
            SpringState::Operational,
            SpringState::Unknown,
            SpringState::Operational,
            SpringState::Operational,
            SpringState::Unknown,
        ];

        let result = find_possible_state_arrangements(&states);
        assert_eq!(result.len(), 4);

        let states = vec![
            SpringState::Operational,
            SpringState::Unknown,
            SpringState::Unknown,
            SpringState::Operational,
            SpringState::Unknown,
            SpringState::Unknown,
        ];

        let result = find_possible_state_arrangements(&states);
        assert_eq!(result.len(), 16);

        let states = vec![
            SpringState::Unknown,
            SpringState::Unknown,
            SpringState::Unknown,
            SpringState::Unknown,
            SpringState::Unknown,
            SpringState::Unknown,
        ];

        let result = find_possible_state_arrangements(&states);
        assert_eq!(result.len(), 64);
    }
}
