use std::collections::HashMap;

use super::{SpringState, SpringStates};

type StateGroupSpaceIndex = (usize, usize, usize);

type Cache = HashMap<StateGroupSpaceIndex, usize>;

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
    pub fn get_states(&self) -> &SpringStates {
        &self.states
    }
    pub fn get_damaged_groups(&self) -> &Vec<usize> {
        &self.damaged_groups
    }

    pub fn find_damaged_combinations(&self) -> usize {
        let mut cache = HashMap::new();

        self.get_damaged_combinations((0, 0, 0), &mut cache)
    }

    fn get_damaged_combinations(&self, indices: StateGroupSpaceIndex, cache: &mut Cache) -> usize {
        let cache_hit = cache.get(&indices);
        if cache_hit.is_some() {
            return *cache_hit.unwrap();
        }

        let states = &self.states;
        let groups = &self.damaged_groups;
        let (state_index, group_index, space) = indices;

        let group_length = *groups.get(group_index).unwrap_or(&0);

        if state_index >= states.len() {
            let is_last_group = group_index == groups.len() - 1;
            let has_room = space == group_length;

            if is_last_group && has_room {
                return 1;
            }

            let past_last_group = group_index >= groups.len();

            if past_last_group && space == 0 {
                return 1;
            }

            return 0;
        }

        let mut combinations = 0;
        let current_state = *states.get(state_index).unwrap();

        for target_state in [SpringState::Damaged, SpringState::Operational] {
            if current_state != target_state && current_state != SpringState::Unknown {
                continue;
            }

            combinations += match target_state {
                SpringState::Operational if space == group_length => {
                    self.get_damaged_combinations((state_index + 1, group_index + 1, 0), cache)
                }
                SpringState::Operational if space == 0 => {
                    self.get_damaged_combinations((state_index + 1, group_index, 0), cache)
                }
                SpringState::Damaged => {
                    self.get_damaged_combinations((state_index + 1, group_index, space + 1), cache)
                }
                _ => 0,
            };
        }

        cache.insert(indices, combinations);

        combinations
    }
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
}
