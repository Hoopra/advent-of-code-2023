mod parse_text_line;

use parse_text_line::TextLine;
use std::fs::read_to_string;

fn main() {
    let file_path = "../input.txt";
    let schematic = read_to_string(file_path).unwrap();

    let score = get_engine_part_score(schematic);

    println!("grand total: {score}") // 528819
}

fn get_engine_part_score(schematic: String) -> u32 {
    let lines: Vec<&str> = schematic.lines().collect();
    let mut sum = 0;

    for (index, line) in lines.iter().enumerate() {
        let previous = get_previous_line(&lines, index);
        let next = get_next_line(&lines, index);

        let text_line = TextLine::new(previous, line, next);

        sum += text_line.get_part_score()
    }

    sum
}

fn get_previous_line<'a>(lines: &'a Vec<&str>, index: usize) -> &'a str {
    if index == 0 {
        return "";
    }

    lines.get(index - 1).unwrap_or(&"")
}

fn get_next_line<'a>(lines: &'a Vec<&str>, index: usize) -> &'a str {
    if index > lines.len() - 1 {
        return "";
    }

    lines.get(index + 1).unwrap_or(&"")
}

#[cfg(test)]
mod test_engine_score {
    use super::*;

    #[test]
    fn calculates_part_score_for_engine() {
        let schematic = String::from("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..");

        assert_eq!(get_engine_part_score(schematic), 4361);
    }
}
