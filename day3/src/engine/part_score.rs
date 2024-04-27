use super::model::{Engine, EngineLine, EngineNumber};

impl EngineLine {
    pub fn get_part_score(&self, previous: Option<&EngineLine>, next: Option<&EngineLine>) -> u32 {
        let numbers = &self.numbers();
        let own_line = Some(self);

        let lines = vec![&own_line, &previous, &next];

        numbers
            .iter()
            .filter(|number| is_number_adjacent_to_symbol(number, &lines))
            .fold(0, |sum: u32, (value, _, _)| sum + value)
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

fn has_symbol_at_positions(line: Option<&EngineLine>, start: usize, end: usize) -> bool {
    if line.is_none() {
        return false;
    }

    line.unwrap()
        .symbols()
        .iter()
        .any(|(_, position)| position >= &start && position <= &end)
}

#[cfg(test)]
mod test_parser {
    use super::*;

    #[test]
    fn determines_if_text_has_symbols_in_positions() {
        let line = EngineLine::new(".......+...38");

        assert_eq!(has_symbol_at_positions(Some(&line), 0, 4), false);
        assert_eq!(has_symbol_at_positions(Some(&line), 5, 8), true);
        assert_eq!(has_symbol_at_positions(Some(&line), 10, 15), false);
    }
}
