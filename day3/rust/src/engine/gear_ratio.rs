use super::model::{Engine, EngineLine, EngineSymbol};

impl EngineLine {
    pub fn get_gear_ratio(&self, previous: Option<&EngineLine>, next: Option<&EngineLine>) -> u32 {
        let symbols = &self.symbols();
        let own_line = Some(self);

        let lines = vec![&own_line, &previous, &next];

        symbols
            .iter()
            .map(|symbol| find_gear_ratio(symbol, &lines))
            .sum()
    }

    fn find_adjacent_numbers(&self, index: usize) -> Vec<u32> {
        let found = self
            .numbers()
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

impl Engine {
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
