// returns longest pattern as (start, length)
pub fn find_repeating_pattern(input: &Vec<usize>) -> Option<(usize, usize)> {
    let input_length = input.len();

    for pattern_start in 0..input_length {
        for length in pattern_start..input_length {
            let pattern_end = pattern_start + length;

            if pattern_end >= input_length {
                continue;
            }

            let pattern = &input[pattern_start..pattern_end];

            if pattern.len() > 0 && is_repeating_pattern(input, pattern, pattern_end) {
                return Some((pattern_start, length));
            }
        }
    }

    None
}

fn is_repeating_pattern(input: &Vec<usize>, pattern: &[usize], offset: usize) -> bool {
    let pattern_first = pattern.get(0).unwrap();
    let mut input_pointer = offset;
    let mut repeats = 0;

    while input_pointer < input.len() {
        let input_number = input.get(input_pointer).unwrap();
        if input_number != pattern_first {
            return false;
        }

        for pattern_pointer in 0..pattern.len() {
            let pattern_number = pattern.get(pattern_pointer).unwrap();
            let input_number = input.get(input_pointer + pattern_pointer);

            match input_number {
                Some(n) if n == pattern_number => {}
                None if repeats >= 1 => return true,
                _ => return false,
            }
        }

        repeats += 1;
        input_pointer += pattern.len();
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_repeating_pattern() {
        let input = vec![0, 1, 0, 1, 2, 3, 4, 5, 6, 1, 2, 3, 4, 5, 6];

        let result = find_repeating_pattern(&input);
        assert_eq!(result, Some((3, 6)));

        let input = vec![0, 0, 1, 2, 0, 1, 2, 1, 2, 0, 1, 2];

        let result = find_repeating_pattern(&input);
        assert_eq!(result, Some((2, 5)));

        let input = vec![0, 0, 1, 2, 0, 1, 2, 3, 1, 2, 0, 1, 2];

        let result = find_repeating_pattern(&input);
        assert_eq!(result, None);
    }

    #[test]
    fn determines_if_pattern_repeats() {
        let input = vec![0, 1, 0, 1, 2, 3, 4, 5, 6, 1, 2, 3, 4, 5, 6];
        let pattern = [1, 2, 3, 4, 5, 6];

        let result = is_repeating_pattern(&input, &pattern, 9);
        assert_eq!(result, true);

        let input = vec![0, 1, 0, 1, 2, 3, 4, 5, 6, 1, 2, 3, 4, 5];
        let pattern = [1, 2, 3, 4, 5, 6];

        let result = is_repeating_pattern(&input, &pattern, 8);
        assert_eq!(result, false);
    }
}
