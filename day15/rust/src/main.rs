use std::fs::read_to_string;

fn main() {
    solve_part_1("../input.txt");
}

fn solve_part_1(file_path: &str) {
    let text = read_to_string(file_path).unwrap();

    let result = compute_hash_values(&text);
    assert_eq!(result, 516657);
}

fn compute_hash_values(input: &str) -> u32 {
    input
        .split(',')
        .fold(0, |value, text| value + compute_hash_value(text))
}

fn compute_hash_value(input: &str) -> u32 {
    input
        .chars()
        .map(|symbol| symbol_to_ascii(symbol))
        .fold(0, |value: u32, next: u32| {
            return ((value + next) * 17) % 256;
        })
}

fn symbol_to_ascii(symbol: char) -> u32 {
    let as_string = String::from(symbol);
    let as_ascii = as_string.as_bytes().get(0).unwrap();
    *as_ascii as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_hash_value() {
        let input = "HASH";

        let result = compute_hash_value(input);
        assert_eq!(result, 52);
    }

    #[test]
    fn calculates_hash_for_multiple_values() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        let result = compute_hash_values(input);
        assert_eq!(result, 1320);
    }
}
