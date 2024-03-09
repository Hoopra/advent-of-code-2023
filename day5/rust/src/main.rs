mod conversion_map;

use conversion_map::ConversionMap;
use std::fs::read_to_string;

fn main() {
    let file_path = "../input.txt";
    let text = read_to_string(file_path).unwrap();

    parse_almanac(text);

    //
}

fn parse_almanac(input: String) {
    let entries: Vec<&str> = input.split("\n\n").collect();

    let seed_list = parse_seed_list(entries.get(0).copied());
    let seed_to_soil = parse_conversion_map(entries.get(1).copied());
    let soil_to_fertilizer = parse_conversion_map(entries.get(2).copied());
    let fertilizer_to_water = parse_conversion_map(entries.get(3).copied());
    let water_to_light = parse_conversion_map(entries.get(4).copied());
    let light_to_temperature = parse_conversion_map(entries.get(5).copied());
    let temperature_to_humidity = parse_conversion_map(entries.get(6).copied());
    let humidity_to_location = parse_conversion_map(entries.get(7).copied());

    let locations: Vec<i64> = seed_list
        .iter()
        .map(|value| {
            let soil = seed_to_soil.convert(*value);
            let fertilizer = soil_to_fertilizer.convert(soil);
            let water = fertilizer_to_water.convert(fertilizer);
            let light = water_to_light.convert(water);
            let temperature = light_to_temperature.convert(light);
            let humidity = temperature_to_humidity.convert(temperature);
            humidity_to_location.convert(humidity)
        })
        .collect();

    let minimum_location_number = locations.iter().min().unwrap_or(&0);
    assert_eq!(*minimum_location_number, 600279879)
}

fn parse_seed_list(text: Option<&str>) -> Vec<i64> {
    text.unwrap_or(&"")
        .split(" ")
        .into_iter()
        .skip(1)
        .map(|value| value.parse::<i64>().unwrap_or(0))
        .collect()
}

fn parse_conversion_map(text: Option<&str>) -> ConversionMap {
    let map_text: String = text
        .unwrap_or(&"")
        .lines()
        .skip(1)
        .fold(String::new(), |previous, next| previous + next + "\n");

    ConversionMap::from_text(map_text)
}
