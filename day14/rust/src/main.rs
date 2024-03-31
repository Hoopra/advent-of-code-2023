mod platform;

use std::fs::read_to_string;

use platform::Platform;

fn main() {
    let text = read_to_string("../input.txt").unwrap();

    let mut platform = Platform::from_text(&text);

    platform.tilt_north();
    let result = platform.calculate_north_load();

    assert_eq!(result, 108144);
}
