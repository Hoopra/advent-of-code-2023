use std::cmp::min;

// start, end
pub type Range = (i64, i64);

#[derive(Debug)]
struct ConversionRange {
    source_start: i64,
    source_end: i64,
    add: i64,
}

impl ConversionRange {
    pub fn new(input: &str) -> ConversionRange {
        let numbers: Vec<&str> = input.split(' ').collect();

        let source_start: i64 = numbers.get(1).unwrap_or(&"0").parse().unwrap();
        let destination_start: i64 = numbers.get(0).unwrap_or(&"0").parse().unwrap();
        let length: i64 = numbers.get(2).unwrap_or(&"0").parse().unwrap();

        ConversionRange {
            add: destination_start - source_start,
            source_start,
            source_end: source_start + length - 1,
        }
    }
}

#[derive(Debug)]
pub struct ConversionMap {
    ranges: Vec<ConversionRange>,
}

impl ConversionMap {
    pub fn from_formatted_text(text: Option<&str>) -> ConversionMap {
        let map_text: String = text
            .unwrap_or(&"")
            .lines()
            .skip(1)
            .fold(String::new(), |previous, next| previous + next + "\n");

        ConversionMap::from_text(map_text)
    }

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
        let range = self
            .ranges
            .iter()
            .find(|range| range.source_start <= input && range.source_end >= input);

        if range.is_none() {
            return input;
        }

        convert_by_range(input, range.unwrap())
    }

    pub fn convert_ranges(&self, ranges: Vec<Range>) -> Vec<Range> {
        ranges
            .iter()
            .map(|range| self.convert_range(*range))
            .flatten()
            .collect()
    }

    pub fn convert_range(&self, input: Range) -> Vec<Range> {
        if self.ranges.len() == 0 {
            return vec![input];
        }

        let mut result: Vec<Range> = vec![];
        let ranges: &Vec<ConversionRange> = &self.ranges;
        let (initial_range_start, initial_range_end) = input;

        let mut current = initial_range_start;

        while current <= initial_range_end {
            let matched_range = find_range_containing_value(ranges, current);

            if matched_range.is_some() {
                let matched_range = matched_range.unwrap();
                let end = min(matched_range.source_end, initial_range_end);

                result.push((
                    convert_by_range(current, matched_range),
                    convert_by_range(end, matched_range),
                ));

                current = end + 1;
                continue;
            }

            let next_range = find_next_range(ranges, current);

            if next_range.is_some() {
                let next_range = next_range.unwrap();
                let end = next_range.source_start - 1;

                result.push((initial_range_start, end));
                current = end + 1;

                continue;
            }

            result.push((current, initial_range_end));
            break;
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
        let range_end = range.source_end;
        range.source_start <= value && range_end >= value
    })
}

fn convert_by_range(value: i64, range: &ConversionRange) -> i64 {
    value + range.add
}

#[cfg(test)]
mod test_conversion_range {
    use super::*;

    #[test]
    fn constructs_conversion_range_from_text() {
        let input = "50 98 2";

        let range = ConversionRange::new(input);

        assert_eq!(range.add, -48);
        assert_eq!(range.source_start, 98);
        assert_eq!(range.source_end, 99);
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

        // input:
        // [90, 91, 92, 93, 94, 95, 96, 97] (90, 97)
        // [98, 99]
        // [100]
        // output:
        // [92, 93, 94, 95, 96, 97, 98, 99] (92, 99)
        // [50, 51] (50, 51)
        // [100] (100, 100)

        assert_eq!(
            map.convert_range((90, 100)),
            vec![(92, 99), (50, 51), (100, 100)]
        );
        assert_eq!(map.convert_range((40, 50)), vec![(40, 49), (52, 52)]);
    }

    #[test]
    fn finds_range_containing_value() {
        let map = ConversionMap::from_text(String::from("50 98 2\n52 50 48"));
        let ranges = &map.ranges;

        let matched = find_range_containing_value(&ranges, 50);
        assert_eq!(matched.unwrap().source_start, 50);
        assert_eq!(matched.unwrap().add, 2);

        let matched = find_range_containing_value(&ranges, 99);
        assert_eq!(matched.unwrap().source_start, 98);
        assert_eq!(matched.unwrap().add, -48);

        let matched = find_range_containing_value(&ranges, 0);
        assert_eq!(matched.is_none(), true);
    }

    #[test]
    fn finds_range_with_next_numerical_start() {
        let map = ConversionMap::from_text(String::from("50 98 2\n52 50 48"));
        let ranges = &map.ranges;

        let next = find_next_range(&ranges, 0);

        assert_eq!(next.unwrap().source_start, 50);
        assert_eq!(next.unwrap().add, 2);
    }
}
