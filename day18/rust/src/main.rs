mod instruction;
mod map_2d;

use instruction::{apply_instructions, construct_path, parse_instructions};
use map_2d::{find_bounding_area, find_enclosed_area, EnclosingArea, PerimeterNode, Position};
use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let text = read_to_string("../input.txt").unwrap();
    solve_part_1(&text);
}

fn solve_part_1(text: &str) {
    let result = find_dug_out_area(text);

    assert_eq!(result.len(), 40714);
}

fn find_dug_out_area(text: &str) -> Vec<Position> {
    let instructions = parse_instructions(&text);

    let positions = apply_instructions(&instructions);

    let perimeter = construct_path(&positions);

    let bounding_area = find_bounding_area(&perimeter);

    let enclosed = find_enclosed_area(&bounding_area, &perimeter);

    draw_area(&bounding_area, &perimeter, &enclosed);

    enclosed
}

pub fn draw_area(
    area: &EnclosingArea,
    positions: &HashMap<Position, PerimeterNode>,
    enclosed: &Vec<Position>,
) {
    let (top_left, bottom_right) = area;
    let (x_min, y_min) = *top_left;
    let (x_max, y_max) = *bottom_right;

    println!("");
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let position = (x, y);

            match positions.get(&position) {
                Some(node) => print!("{}", node.symbol),
                _ => match enclosed.contains(&position) {
                    true => print!("#"),
                    _ => print!("."),
                },
            }
        }
        println!("");
    }
    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determines_dug_out_area() {
        let input = "R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)\nD 2 (#d2c081)\nR 2 (#59c680)\nD 2 (#411b91)\nL 5 (#8ceee2)\nU 2 (#caa173)\nL 1 (#1b58a2)\nU 2 (#caa171)\nR 2 (#7807d2)\nU 3 (#a77fa3)\nL 2 (#015232)\nU 2 (#7a21e3)";

        let result = find_dug_out_area(input);

        assert_eq!(result.len(), 62);
    }
}
