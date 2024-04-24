use crate::position_2d::Position;

// https://en.wikipedia.org/wiki/Shoelace_formula
pub fn shoelace_area(perimeter_vertices: &Vec<Position>) -> f64 {
    let first = perimeter_vertices.first().unwrap();

    let result: isize = perimeter_vertices
        .iter()
        .enumerate()
        .map(|(index, point)| {
            let next = perimeter_vertices.get(index + 1).unwrap_or(first);
            determinant(point, next)
        })
        .sum();

    (result as f64) / 2.0
}

// https://en.wikipedia.org/wiki/Pick%27s_theorem

// A = i + b/2 - 1
// i = A - b/2 + 1
// i + b = A + b/2 + 1

pub fn picks_all_points(area: usize, number_perimeter_points: usize) -> usize {
    area + number_perimeter_points / 2 + 1
}

fn determinant(p1: &Position, p2: &Position) -> isize {
    let (x1, y1) = *p1;
    let (x2, y2) = *p2;

    x1 * y2 - y1 * x2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn applies_shoelace_formula() {
        let vertices = vec![(1, 6), (3, 1), (7, 2), (4, 4), (8, 5)];

        let area = shoelace_area(&vertices);
        assert_eq!(area, 16.5);
    }
}
