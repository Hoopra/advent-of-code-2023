pub fn compute_hash_values(input: &str) -> usize {
    input
        .split(',')
        .fold(0, |value, text| value + compute_hash_value(text))
}

pub fn compute_hash_value(input: &str) -> usize {
    input
        .chars()
        .map(|symbol| symbol_to_ascii(symbol))
        .fold(0, |value: usize, next: usize| {
            return ((value + next) * 17) % 256;
        })
}

fn symbol_to_ascii(symbol: char) -> usize {
    let as_string = String::from(symbol);
    let as_ascii = as_string.as_bytes().get(0).unwrap();
    *as_ascii as usize
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
