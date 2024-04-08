mod cycle;
mod platform;

use std::fs::read_to_string;

use platform::Platform;

fn main() {
    solve_part_1("../input.txt");
    solve_part_2("../input.txt");
}

fn solve_part_1(file_path: &str) {
    let text = read_to_string(file_path).unwrap();

    let mut platform = Platform::from_text(&text);

    platform.tilt_north();
    let result = platform.calculate_north_load();

    assert_eq!(result, 108144);
}

fn solve_part_2(file_path: &str) {
    let text = read_to_string(file_path).unwrap();

    let mut platform = Platform::from_text(&text);

    let result = platform.calculate_load_after_cycles(1_000_000_000);

    assert_eq!(result, 108404);
}
