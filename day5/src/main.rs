mod conversion_map;
mod seed_list;
mod seed_ranges;

use seed_list::parse_almanac_seed_list;
use seed_ranges::parse_almanac_seed_ranges;
use std::fs::read_to_string;

fn main() {
    let file_path = "../input.txt";
    let text = read_to_string(file_path).unwrap();

    parse_almanac_seed_list(&text);
    parse_almanac_seed_ranges(&text);
}
