pub fn create_permutations(unknowns: usize) -> Vec<Vec<bool>> {
    if unknowns == 0 {
        return vec![];
    }

    create_permutation_with_size(unknowns, None)
}

pub fn create_permutation_with_size(
    target: usize,
    previous: Option<Vec<Vec<bool>>>,
) -> Vec<Vec<bool>> {
    let next = match previous {
        None => vec![vec![true], vec![false]],
        Some(previous) => {
            let mut result = vec![];

            for start_value in [true, false] {
                for previous_values in previous.iter() {
                    let mut inner = vec![start_value];

                    previous_values.iter().for_each(|value| {
                        inner.push(*value);
                    });

                    result.push(inner);
                }
            }

            result
        }
    };

    if next.len() >= (2 as usize).pow(target as u32) {
        return next;
    }

    create_permutation_with_size(target, Some(next))
}
