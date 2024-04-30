use std::fs::read_to_string;

type NumberLine = Vec<i32>;

fn main() {
    let file_path = "input.txt";
    let text = read_to_string(file_path).unwrap();

    extrapolate_last(&text);
    extrapolate_first(&text)
}

fn extrapolate_last(text: &str) {
    let sum_total: i32 = text
        .lines()
        .map(|text_line| {
            let number_line = text_line
                .split_whitespace()
                .map(|number| number.parse::<i32>().unwrap())
                .collect();
            find_next_last_in_number_line(&number_line)
        })
        .sum();

    assert_eq!(sum_total, 1898776583);
}

fn extrapolate_first(text: &str) {
    let sum_total: i32 = text
        .lines()
        .map(|text_line| {
            let number_line = text_line
                .split_whitespace()
                .map(|number| number.parse::<i32>().unwrap())
                .collect();
            find_next_first_in_number_line(&number_line)
        })
        .sum();

    assert_eq!(sum_total, 1100);
}

fn find_next_last_in_number_line(line: &NumberLine) -> i32 {
    let lines = unwrap_line(line);

    lines
        .iter()
        .rev()
        .map(|line| line.iter().last().unwrap())
        .sum()
}

fn find_next_first_in_number_line(line: &NumberLine) -> i32 {
    let lines = unwrap_line(line);

    lines
        .iter()
        .rev()
        .enumerate()
        .map(|(index, line)| (index, line.first().unwrap()))
        .fold(0, |accumulator, (index, next)| {
            if index == 0 {
                return *next;
            }
            next - accumulator
        })
}

fn unwrap_line(line: &NumberLine) -> Vec<NumberLine> {
    let mut lines: Vec<NumberLine> = vec![line.to_vec()];
    let mut lines_index = 0;

    loop {
        let line = lines.get(lines_index).unwrap();
        let next_line = find_next_line_by_difference(line);

        lines.push(next_line.to_vec());

        if next_line.iter().all(|value| value == &0) {
            break;
        }

        lines_index += 1;
    }

    lines
}

fn find_next_line_by_difference(line: &NumberLine) -> NumberLine {
    line.iter()
        .skip(1)
        .enumerate()
        .map(|(index, value)| {
            let previous = line.get(index).unwrap();
            value - previous
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_next_line_in_sequence() {
        let input = vec![0, 3, 6, 9, 12, 15];

        let result = find_next_line_by_difference(&input);
        assert_eq!(result, vec![3, 3, 3, 3, 3]);

        let result = find_next_line_by_difference(&result);
        assert_eq!(result, vec![0, 0, 0, 0]);
    }

    #[test]
    fn unwraps_line() {
        let input = vec![0, 3, 6, 9, 12, 15];

        let result = unwrap_line(&input);
        assert_eq!(
            result,
            vec![input.to_vec(), vec![3, 3, 3, 3, 3], vec![0, 0, 0, 0]]
        );
    }

    #[test]
    fn calculates_next_last_number_in_line() {
        let input = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(find_next_last_in_number_line(&input), 18);

        let input = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(find_next_last_in_number_line(&input), 28);

        let input = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(find_next_last_in_number_line(&input), 68);
    }

    #[test]
    fn calculates_next_first_number_in_line() {
        let input = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(find_next_first_in_number_line(&input), -3);

        let input = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(find_next_first_in_number_line(&input), 0);

        let input = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(find_next_first_in_number_line(&input), 5);
    }
}
