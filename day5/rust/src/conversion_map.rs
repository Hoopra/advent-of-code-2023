#[derive(Debug)]
struct ConversionRange {
    destination_start: i64,
    source_start: i64,
    length: i64,
}

impl ConversionRange {
    pub fn new(input: &str) -> ConversionRange {
        let numbers: Vec<&str> = input.split(' ').collect();

        ConversionRange {
            destination_start: numbers.get(0).unwrap_or(&"0").parse().unwrap(),
            source_start: numbers.get(1).unwrap_or(&"0").parse().unwrap(),
            length: numbers.get(2).unwrap_or(&"0").parse().unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct ConversionMap {
    ranges: Vec<ConversionRange>,
}

impl ConversionMap {
    pub fn from_text(text: String) -> ConversionMap {
        let ranges = text
            .lines()
            .filter(|line| line.split(' ').collect::<Vec<&str>>().len() == 3)
            .map(|line| ConversionRange::new(line))
            .collect();

        ConversionMap { ranges }
    }
}

impl ConversionMap {
    pub fn convert(&self, input: i64) -> i64 {
        let range = self.ranges.iter().find(|range| {
            range.source_start <= input && range.source_start + range.length >= input
        });

        if range.is_none() {
            return input;
        }

        let ConversionRange {
            destination_start,
            source_start,
            ..
        } = range.unwrap();

        input + (destination_start - source_start)
    }
}

#[cfg(test)]
mod test_conversion_range {
    use super::*;

    #[test]
    fn constructs_conversion_range_from_text() {
        let input = "50 98 2";

        let ConversionRange {
            destination_start,
            source_start,
            length,
        } = ConversionRange::new(input);

        assert_eq!(destination_start, 50);
        assert_eq!(source_start, 98);
        assert_eq!(length, 2);
    }
}

#[cfg(test)]
mod test_conversion_map {
    use super::*;

    #[test]
    fn constructed_conversion_map_converts_correctly() {
        let input = String::from("50 98 2\n52 50 48");

        let map = ConversionMap::from_text(input);

        assert_eq!(map.convert(99), 51);
        assert_eq!(map.convert(60), 62);
    }
}
