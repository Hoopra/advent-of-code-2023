mod instruction;
mod math;
mod position_2d;

use crate::math::{picks_all_points, shoelace_area};
use instruction::{
    count_perimeter_points, instructions_to_vertices, parse_instructions,
    parse_instructions_by_color, DigInstruction,
};
use std::fs::read_to_string;

fn main() {
    let text = read_to_string("../input.txt").unwrap();

    solve_part_1(&text);

    solve_part_2(&text);
}

fn solve_part_1(text: &str) {
    let instructions = parse_instructions(&text);
    let result = find_dug_out_area_by_formula(&instructions);

    assert_eq!(result, 40714);
}

fn solve_part_2(text: &str) {
    let instructions = parse_instructions_by_color(&text);

    let result = find_dug_out_area_by_formula(&instructions);
    assert_eq!(result, 129849166997110)
}

fn find_dug_out_area_by_formula(instructions: &Vec<DigInstruction>) -> usize {
    let vertices = instructions_to_vertices(instructions);

    let perimeter_points = count_perimeter_points(&instructions);
    let area = shoelace_area(&vertices);

    picks_all_points(area as usize, perimeter_points)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determines_dug_out_area() {
        let text = "R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)\nD 2 (#d2c081)\nR 2 (#59c680)\nD 2 (#411b91)\nL 5 (#8ceee2)\nU 2 (#caa173)\nL 1 (#1b58a2)\nU 2 (#caa171)\nR 2 (#7807d2)\nU 3 (#a77fa3)\nL 2 (#015232)\nU 2 (#7a21e3)";

        let instructions = parse_instructions(text);
        let result = find_dug_out_area_by_formula(&instructions);

        assert_eq!(result, 62);
    }

    #[test]
    fn determines_dug_out_area_for_color_instructions() {
        let text = "R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)\nD 2 (#d2c081)\nR 2 (#59c680)\nD 2 (#411b91)\nL 5 (#8ceee2)\nU 2 (#caa173)\nL 1 (#1b58a2)\nU 2 (#caa171)\nR 2 (#7807d2)\nU 3 (#a77fa3)\nL 2 (#015232)\nU 2 (#7a21e3)";

        let instructions = parse_instructions_by_color(&text);

        let result = find_dug_out_area_by_formula(&instructions);

        assert_eq!(result, 952408144115);
    }
}
