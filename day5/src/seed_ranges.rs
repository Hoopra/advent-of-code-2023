use crate::conversion_map::Range;
use std::cmp::min;

use crate::conversion_map::ConversionMap;

pub fn parse_almanac_seed_ranges(input: &String) {
    let entries: Vec<&str> = input.split("\n\n").collect();

    let seed_list = parse_seed_ranges(entries.get(0).copied());
    let seed_to_soil = ConversionMap::from_formatted_text(entries.get(1).copied());
    let soil_to_fertilizer = ConversionMap::from_formatted_text(entries.get(2).copied());
    let fertilizer_to_water = ConversionMap::from_formatted_text(entries.get(3).copied());
    let water_to_light = ConversionMap::from_formatted_text(entries.get(4).copied());
    let light_to_temperature = ConversionMap::from_formatted_text(entries.get(5).copied());
    let temperature_to_humidity = ConversionMap::from_formatted_text(entries.get(6).copied());
    let humidity_to_location = ConversionMap::from_formatted_text(entries.get(7).copied());

    let locations: Vec<Range> = seed_list
        .iter()
        .flat_map(|value| {
            let soil = seed_to_soil.convert_range(*value);
            let fertilizer = soil_to_fertilizer.convert_ranges(soil);
            let water = fertilizer_to_water.convert_ranges(fertilizer);
            let light = water_to_light.convert_ranges(water);
            let temperature = light_to_temperature.convert_ranges(light);
            let humidity = temperature_to_humidity.convert_ranges(temperature);
            humidity_to_location.convert_ranges(humidity)
        })
        .collect();

    let minimum_location_number = locations
        .iter()
        .map(|(start, end)| min(start, end))
        .min()
        .unwrap_or(&0);

    assert_eq!(*minimum_location_number, 20191102);
}

fn parse_seed_ranges(text: Option<&str>) -> Vec<Range> {
    let mut ranges: Vec<Range> = vec![];
    let mut start = None;

    for number_text in text.unwrap_or(&"").split(" ").into_iter().skip(1) {
        let number = number_text.parse::<i64>().unwrap_or(0);

        if start.is_none() {
            start = Some(number);
            continue;
        }

        let start_value = start.unwrap();
        ranges.push((start_value, start_value + number));

        start = None;
    }

    ranges
}
