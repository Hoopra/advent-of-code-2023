use std::cmp::{max, min};

pub struct TextLine<'a> {
    previous: &'a str,
    text: &'a str,
    next: &'a str,
}

type NumberWithLength = (u32, usize, usize);

impl<'a> TextLine<'a> {
    pub fn new(previous: &'a str, current: &'a str, next: &'a str) -> TextLine<'a> {
        TextLine {
            previous,
            text: current,
            next,
        }
    }
}

impl<'a> TextLine<'a> {
    pub fn get_part_score(&'a self) -> u32 {
        let numbers = self.find_numbers();
        let lines = vec![self.previous, self.text, self.next];

        numbers
            .iter()
            .filter(|number| is_number_adjacent_to_symbol(number, &lines))
            .fold(0, |sum: u32, (value, _, _)| sum + value)
    }

    pub fn find_numbers(&self) -> Vec<NumberWithLength> {
        find_numbers_in_text(self.text)
    }
}

fn is_number_adjacent_to_symbol(number: &NumberWithLength, lines: &Vec<&str>) -> bool {
    let (_, start, length) = number;

    let start = match *start >= 2 {
        true => start - 2,
        false => 0,
    };

    let end = start + length + 2;

    let found = lines
        .iter()
        .any(|line| has_symbol_at_positions(line, start, end));

    found
}

fn find_numbers_in_text(text: &str) -> Vec<NumberWithLength> {
    let mut result = vec![];
    let mut cursor = 0;
    let length = text.len();

    let characters: Vec<char> = text.chars().collect();

    while cursor < text.len() {
        let character = characters.get(cursor);
        if character.is_none() {
            break;
        }

        let character = character.unwrap();
        if character.is_ascii_digit() {
            let end = min(cursor + 5, length);
            let (value, length) = extract_number(&text[cursor..end]);

            result.push((value, cursor + 1, length));
            cursor += length;
            continue;
        }

        cursor += 1;
    }

    result
}

fn has_symbol_at_positions(text: &str, start: usize, end: usize) -> bool {
    let length = text.len();

    if length <= 0 || start > end || start > length - 1 {
        return false;
    }

    let start = max(start, 0);
    let end = min(end, length);

    text[start..end].chars().any(is_symbol)
}

fn is_symbol(character: char) -> bool {
    !character.is_ascii_digit() && character != '.'
}

fn extract_number(text: &str) -> (u32, usize) {
    let mut digits: String = String::new();
    let mut length: usize = 0;

    for character in text.chars() {
        if !character.is_ascii_digit() {
            break;
        }

        digits.push(character);
        length += 1;
    }

    let value = digits.parse::<u32>();

    match value {
        Ok(value) => (value, length),
        Err(_) => (0, 0),
    }
}

#[cfg(test)]
mod test_parser {
    use super::*;

    #[test]
    fn extracts_number_from_string_discarding_tail() {
        assert_eq!(extract_number("169...."), (169, 3));
        assert_eq!(extract_number(".69...."), (0, 0));
    }

    #[test]
    fn finds_numbers_in_text() {
        let input_text = "....737.....608..........362...336....642....606";
        let result = find_numbers_in_text(input_text);

        assert_eq!(
            result,
            vec![
                (737, 5, 3),
                (608, 13, 3),
                (362, 26, 3),
                (336, 32, 3),
                (642, 39, 3),
                (606, 46, 3)
            ]
        );
    }

    #[test]
    fn determines_if_text_has_symbols_in_positions() {
        let text = ".......+...38";

        assert_eq!(has_symbol_at_positions(text, 0, 4), false);
        assert_eq!(has_symbol_at_positions(text, 5, 8), true);
        assert_eq!(has_symbol_at_positions(text, 10, 15), false);
    }

    #[test]
    fn finds_part_score_for_text_line() {
        let text_line = TextLine::new(
            // intended to preserve order
            "..592.....",
            "......755.",
            "...$.*....",
        );

        assert_eq!(text_line.get_part_score(), 755);
    }
}
