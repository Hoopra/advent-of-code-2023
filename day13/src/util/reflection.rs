type RowPair = (usize, usize);

pub fn find_reflection(pairs: &Vec<RowPair>, max_index: usize) -> Option<usize> {
    let candidates = find_reflections(pairs, max_index);

    candidates.get(0).map(|value| *value)
}

pub fn find_reflections(pairs: &Vec<RowPair>, max_index: usize) -> Vec<usize> {
    pairs
        .iter()
        .filter_map(|(index_a, index_b)| {
            if index_b - index_a != 1 {
                return None;
            }

            match is_reflection(*index_a, pairs, max_index) {
                false => None,
                true => Some(*index_b),
            }
        })
        .collect()
}

fn is_reflection(index: usize, pairs: &Vec<RowPair>, max_index: usize) -> bool {
    for i in 1..(max_index - index) {
        let upper = index + i + 1;

        if i > index || upper > max_index {
            continue;
        }

        if !pairs.contains(&(index - i, upper)) {
            return false;
        }
    }

    true
}
