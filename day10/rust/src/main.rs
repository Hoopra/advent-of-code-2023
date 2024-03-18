mod pipe;
mod pipe_loop;
mod pipe_map;

use pipe_loop::find_farthest_connected_pipe;
use std::fs::read_to_string;

fn main() {
    let file_path = "../input.txt";
    let text = read_to_string(file_path).unwrap();

    let most_steps = find_farthest_connected_pipe(&text);
    assert_eq!(most_steps, 6923);
}
