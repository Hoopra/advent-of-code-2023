use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PartCharacteristic {
    ExtremelyCool,
    Musical,
    Aerodynamic,
    Shiny,
}

impl PartCharacteristic {
    pub fn from_symbol(symbol: char) -> PartCharacteristic {
        match symbol {
            'a' => PartCharacteristic::Aerodynamic,
            'x' => PartCharacteristic::ExtremelyCool,
            's' => PartCharacteristic::Shiny,
            'm' | _ => PartCharacteristic::Musical,
        }
    }
}

#[derive(Debug)]
pub struct Part {
    pub characteristics: HashMap<PartCharacteristic, u32>,
}

impl Part {
    pub fn from_text(text: &str) -> Part {
        let text: String = text.chars().skip(1).take(text.len() - 2).collect();

        let mut characteristics = HashMap::new();

        text.split(',').for_each(|inner| {
            let symbol = inner.chars().nth(0).unwrap();
            let key = PartCharacteristic::from_symbol(symbol);

            let value = inner.chars().skip(2).collect::<String>().parse().unwrap();

            characteristics.insert(key, value);
        });

        Part { characteristics }
    }
}

impl Part {
    pub fn get_rating(&self) -> u32 {
        self.characteristics.values().sum()
    }
}

pub fn parse_parts(text: &str) -> Vec<Part> {
    text.lines().map(Part::from_text).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_part_from_text() {
        let part = Part::from_text("{x=1072,m=503,a=517,s=1973}");

        assert_eq!(part.characteristics.len(), 4);
    }
}
