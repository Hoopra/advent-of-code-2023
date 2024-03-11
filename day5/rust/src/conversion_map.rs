use std::cmp::min;

// start, end
type Range = (i64, i64);

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

        convert_by_range(input, range.unwrap())
    }

    pub fn convert_range(&self, input: Range) -> Vec<Range> {
        if self.ranges.len() == 0 {
            return vec![input];
        }

        let mut result: Vec<Range> = vec![];
        let ranges: &Vec<ConversionRange> = &self.ranges;
        let (initial_range_start, initial_range_end) = input;

        let mut current = initial_range_start;

        while current < initial_range_end {
            let matched_range = find_range_containing_value(ranges, current);

            if matched_range.is_some() {
                let matched_range = matched_range.unwrap();
                let end = min(
                    matched_range.source_start + matched_range.length - 1,
                    initial_range_end,
                );

                result.push((
                    convert_by_range(current, matched_range),
                    convert_by_range(end, matched_range),
                ));

                current = end + 1;
                continue;
            }

            let next_range = find_next_range(ranges, current);

            if next_range.is_none() {
                result.push((initial_range_start, initial_range_end));
                current = initial_range_end;
                continue;
            }

            let next_range = next_range.unwrap();

            let end = next_range.source_start - 1;

            result.push((initial_range_start, end));
            current = end;
        }

        result
    }
}

fn find_next_range(ranges: &Vec<ConversionRange>, start: i64) -> Option<&ConversionRange> {
    ranges.iter().fold(None, |previous, next| {
        if next.source_start < start {
            return previous;
        }

        if previous.is_none() {
            return Some(next);
        }

        let previous = previous.unwrap();

        let previous_difference = previous.source_start - start;
        let next_difference = next.source_start - start;

        match next_difference < previous_difference {
            true => Some(next),
            false => Some(previous),
        }
    })
}

fn find_range_containing_value(
    ranges: &Vec<ConversionRange>,
    value: i64,
) -> Option<&ConversionRange> {
    ranges.iter().find(|range| {
        let range_end = range.source_start + range.length;
        range.source_start <= value && range_end >= value
    })
}

fn convert_by_range(value: i64, range: &ConversionRange) -> i64 {
    let ConversionRange {
        destination_start,
        source_start,
        ..
    } = range;

    value + (destination_start - source_start)
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

    #[test]
    fn correctly_subdivides_input_range() {
        let input = String::from("50 98 2\n52 50 48");

        let map = ConversionMap::from_text(input);

        let range = (90, 100);
        assert_eq!(map.convert_range(range), vec![(92, 99), (50, 51)]);
    }

    #[test]
    fn finds_range_containing_value() {
        let map = ConversionMap::from_text(String::from("50 98 2\n52 50 48"));
        let ranges = &map.ranges;

        let matched = find_range_containing_value(&ranges, 50);
        assert_eq!(matched.unwrap().source_start, 50);
        assert_eq!(matched.unwrap().destination_start, 52);

        let matched = find_range_containing_value(&ranges, 99);
        assert_eq!(matched.unwrap().source_start, 98);
        assert_eq!(matched.unwrap().destination_start, 50);

        let matched = find_range_containing_value(&ranges, 0);
        assert_eq!(matched.is_none(), true);
    }

    #[test]
    fn finds_range_with_next_numerical_start() {
        let map = ConversionMap::from_text(String::from("50 98 2\n52 50 48"));
        let ranges = &map.ranges;

        let next = find_next_range(&ranges, 0);

        assert_eq!(next.unwrap().source_start, 50);
        assert_eq!(next.unwrap().destination_start, 52);
    }
}
