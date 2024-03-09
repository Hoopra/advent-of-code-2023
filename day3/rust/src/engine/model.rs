use std::cmp::min;

// character, position
pub type EngineSymbol = (char, usize);

// value, start, length
pub type EngineNumber = (u32, usize, usize);

pub struct EngineLine {
    numbers: Vec<EngineNumber>,
    symbols: Vec<EngineSymbol>,
}

impl EngineLine {
    pub fn new(text: &str) -> EngineLine {
        EngineLine {
            numbers: find_numbers_in_text(text),
            symbols: find_symbols_in_text(text),
        }
    }
}

impl EngineLine {
    pub fn numbers(&self) -> &Vec<EngineNumber> {
        &self.numbers
    }

    pub fn symbols(&self) -> &Vec<EngineSymbol> {
        &self.symbols
    }
}

pub struct Engine {
    pub lines: Vec<EngineLine>,
}

impl Engine {
    pub fn new(text: String) -> Engine {
        Engine {
            lines: text.lines().map(EngineLine::new).collect(),
        }
    }
}

impl Engine {
    pub fn get_previous_line<'a>(&self, index: usize) -> Option<&EngineLine> {
        match index > 0 {
            true => self.lines.get(index - 1),
            false => None,
        }
    }
}

fn find_numbers_in_text(text: &str) -> Vec<EngineNumber> {
    let mut result = vec![];
    let mut cursor = 0;
    let length = text.len();

    let characters: Vec<char> = text.chars().collect();

    while cursor < length {
        let character = characters.get(cursor);
        if character.is_none() {
            break;
        }

        let character = character.unwrap();
        if character.is_ascii_digit() {
            let end = min(cursor + 5, length);
            let (value, length) = extract_number(&text[cursor..end]);

            result.push((value, cursor, length));
            cursor += length;
            continue;
        }

        cursor += 1;
    }

    result
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

fn find_symbols_in_text(text: &str) -> Vec<EngineSymbol> {
    let mut result = vec![];
    let mut cursor = 0;

    let characters: Vec<char> = text.chars().collect();

    while cursor < text.len() {
        let character = characters.get(cursor);
        if character.is_none() {
            break;
        }

        let character = *character.unwrap();
        if is_symbol(character) {
            result.push((character, cursor));
        }

        cursor += 1;
    }

    result
}

pub fn is_symbol(character: char) -> bool {
    !character.is_ascii_digit() && character != '.'
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
                (737, 4, 3),
                (608, 12, 3),
                (362, 25, 3),
                (336, 31, 3),
                (642, 38, 3),
                (606, 45, 3)
            ]
        );
    }

    #[test]
    fn finds_symbols_in_text() {
        let input_text = ".*.......405...*..........*......%";
        let result = find_symbols_in_text(input_text);

        assert_eq!(result, vec![('*', 1), ('*', 15), ('*', 26), ('%', 33),]);
    }

    #[test]
    fn finds_part_score_for_text_line() {
        let text = "..592.....\n......755.\n...$.*....";
        let engine = Engine::new(String::from(text));

        assert_eq!(engine.calculate_part_score(), 755);
    }

    #[test]
    fn calculates_part_score_for_engine() {
        let schematic = String::from("467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..");
        let engine = Engine::new(schematic);

        assert_eq!(engine.calculate_part_score(), 4361);
    }

    #[test]
    fn finds_gear_ratio() {
        let schematic = String::from("467..114..\n..._......\n..35..633.\n......#...\n617_......\n.....+.58.\n..592.....\n......755.\n...$._....\n.664.598..");

        let engine = Engine::new(schematic);

        assert_eq!(engine.calculate_gear_ratio(), 467835);
    }
}
