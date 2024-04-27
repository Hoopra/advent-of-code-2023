use crate::position_2d::Direction;

use super::DigInstruction;

impl DigInstruction {
    pub fn from_text(text: &str) -> Self {
        let components: Vec<&str> = text.split_whitespace().collect();

        let direction = Direction::from(*components.get(0).unwrap());
        let steps = components.get(1).unwrap().parse().unwrap();

        DigInstruction { direction, steps }
    }

    pub fn from_text_by_color(text: &str) -> Self {
        let components: Vec<&str> = text.split_whitespace().collect();

        let color = components
            .get(2)
            .unwrap()
            .to_string()
            .chars()
            .skip(1)
            .take(7)
            .collect();

        let (direction, steps) = convert_color_to_instruction(color);

        DigInstruction { direction, steps }
    }
}

fn convert_color_to_instruction(color: String) -> (Direction, isize) {
    let color_components: Vec<char> = color.chars().collect();

    let direction = match *color_components.last().unwrap() {
        '0' => Direction::Right,
        '1' => Direction::Down,
        '2' => Direction::Left,
        '3' | _ => Direction::Up,
    };

    let hex_steps: String = color_components
        .iter()
        .skip(1)
        .take(5)
        .map(|value| format!("{}", value))
        .collect();

    let steps = isize::from_str_radix(&hex_steps, 16).unwrap();

    (direction, steps)
}

pub fn parse_instructions(input: &str) -> Vec<DigInstruction> {
    input.lines().map(DigInstruction::from_text).collect()
}

pub fn parse_instructions_by_color(input: &str) -> Vec<DigInstruction> {
    input
        .lines()
        .map(DigInstruction::from_text_by_color)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_instruction() {
        let input = "R 6 (#70c710)";

        let result = DigInstruction::from_text(input);

        assert_eq!(result.direction, Direction::Right);
        assert_eq!(result.steps, 6);
    }

    #[test]
    fn parses_instruction_using_color() {
        let input = "R 6 (#70c710)";

        let result = DigInstruction::from_text_by_color(input);

        assert_eq!(result.direction, Direction::Right);
        assert_eq!(result.steps, 461937);
    }
}
