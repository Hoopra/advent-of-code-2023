use std::cmp::min;

// character, position
type EngineSymbol = (char, usize);

// value, start, length
type EngineNumber = (u32, usize, usize);

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
    pub fn get_part_score(&self, previous: Option<&EngineLine>, next: Option<&EngineLine>) -> u32 {
        let numbers = &self.numbers;
        let own_line = Some(self);

        let lines = vec![&own_line, &previous, &next];

        numbers
            .iter()
            .filter(|number| is_number_adjacent_to_symbol(number, &lines))
            .fold(0, |sum: u32, (value, _, _)| sum + value)
    }

    pub fn get_gear_ratio(&self, previous: Option<&EngineLine>, next: Option<&EngineLine>) -> u32 {
        let symbols = &self.symbols;
        let own_line = Some(self);

        let lines = vec![&own_line, &previous, &next];

        symbols
            .iter()
            .map(|symbol| find_gear_ratio(symbol, &lines))
            .sum()
    }

    fn find_adjacent_numbers(&self, index: usize) -> Vec<u32> {
        let found = self
            .numbers
            .iter()
            .filter(|(_, start, length)| {
                let start = *start;
                let end = start + length - 1;

                end + 1 >= index && start <= index + 1
            })
            .map(|(value, _, _)| *value)
            .collect();

        found
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
    pub fn calculate_part_score(&self) -> u32 {
        let lines = &self.lines;
        let mut sum = 0;

        for (index, line) in lines.iter().enumerate() {
            let previous = self.get_previous_line(index);
            let next = lines.get(index + 1);

            let score = line.get_part_score(previous, next);

            sum += score
        }

        sum
    }

    fn get_previous_line<'a>(&self, index: usize) -> Option<&EngineLine> {
        match index > 0 {
            true => self.lines.get(index - 1),
            false => None,
        }
    }

    pub fn calculate_gear_ratio(&self) -> u32 {
        let lines = &self.lines;
        let mut sum = 0;

        for (index, line) in lines.iter().enumerate() {
            let previous = self.get_previous_line(index);
            let next = lines.get(index + 1);

            let ratio = line.get_gear_ratio(previous, next);

            sum += ratio
        }

        sum
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

fn is_symbol(character: char) -> bool {
    !character.is_ascii_digit() && character != '.'
}

fn is_number_adjacent_to_symbol(number: &EngineNumber, lines: &Vec<&Option<&EngineLine>>) -> bool {
    let (_, start, length) = number;

    let start = match *start >= 1 {
        true => start - 1,
        false => 0,
    };

    let end = start + length + 1;

    let found = lines
        .iter()
        .any(|line| has_symbol_at_positions(line.as_deref(), start, end));

    found
}

fn find_gear_ratio((_, index): &EngineSymbol, lines: &Vec<&Option<&EngineLine>>) -> u32 {
    let numbers = find_adjacent_numbers(*index, lines);

    match numbers.len() == 2 {
        false => 0,
        true => numbers.iter().fold(1, |previous, next| previous * next),
    }
}

fn find_adjacent_numbers(index: usize, lines: &Vec<&Option<&EngineLine>>) -> Vec<u32> {
    lines
        .iter()
        .flat_map(|line| find_numbers_in_line(index, **line))
        .collect()
}

fn find_numbers_in_line(index: usize, line: Option<&EngineLine>) -> Vec<u32> {
    if line.is_none() {
        return vec![];
    }

    line.unwrap().find_adjacent_numbers(index)
}

fn has_symbol_at_positions(line: Option<&EngineLine>, start: usize, end: usize) -> bool {
    if line.is_none() {
        return false;
    }

    line.unwrap()
        .symbols
        .iter()
        .any(|(_, position)| position >= &start && position <= &end)
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
    fn determines_if_text_has_symbols_in_positions() {
        let line = EngineLine::new(".......+...38");

        assert_eq!(has_symbol_at_positions(Some(&line), 0, 4), false);
        assert_eq!(has_symbol_at_positions(Some(&line), 5, 8), true);
        assert_eq!(has_symbol_at_positions(Some(&line), 10, 15), false);
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
