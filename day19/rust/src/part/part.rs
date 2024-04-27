use super::PartFeature;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Part {
    pub features: HashMap<PartFeature, u32>,
}

impl Part {
    pub fn from_text(text: &str) -> Part {
        let text: String = text.chars().skip(1).take(text.len() - 2).collect();

        let mut features = HashMap::new();

        text.split(',').for_each(|inner| {
            let symbol = inner.chars().nth(0).unwrap();
            let key = PartFeature::from_symbol(symbol);

            let value = inner.chars().skip(2).collect::<String>().parse().unwrap();

            features.insert(key, value);
        });

        Part { features }
    }
}

impl Part {
    pub fn get_rating(&self) -> u32 {
        self.features.values().sum()
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

        assert_eq!(part.features.len(), 4);
    }
}
