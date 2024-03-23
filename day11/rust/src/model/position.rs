pub type Position = (usize, usize);

pub fn distance_between_positions(a: &Position, b: &Position) -> usize {
    let (xa, ya) = a;
    let (xb, yb) = b;

    let x = steps_between_coordinates(xa, xb);
    let y = steps_between_coordinates(ya, yb);

    x + y
}

fn steps_between_coordinates(a: &usize, b: &usize) -> usize {
    match a > b {
        true => a - b,
        _ => b - a,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_distance_between_positions() {
        let distance = distance_between_positions(&(1, 6), &(5, 11));
        assert_eq!(distance, 9);

        let distance = distance_between_positions(&(4, 0), &(9, 10));
        assert_eq!(distance, 15);

        let distance = distance_between_positions(&(0, 11), &(5, 11));
        assert_eq!(distance, 5);

        let distance = distance_between_positions(&(0, 2), &(12, 7));
        assert_eq!(distance, 17);
    }
}
