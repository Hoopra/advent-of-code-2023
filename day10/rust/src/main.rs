mod enclosed_tiles;
mod model;

use enclosed_tiles::find_enclosed_tiles_in_map;
use model::{PipeLoop, PipeMap};
use std::fs::read_to_string;

fn main() {
    let file_path = "../input.txt";
    let text = read_to_string(file_path).unwrap();

    let most_steps = find_farthest_connected_pipe(&text);
    assert_eq!(most_steps, 6923);

    let enclosed_tiles = find_enclosed_tiles_in_map(&text);
    assert_eq!(enclosed_tiles.len(), 529);
}

pub fn find_farthest_connected_pipe(input: &str) -> u32 {
    let map = PipeMap::from_text(input);
    let pipe_loop = PipeLoop::from_map(&map);

    ((pipe_loop.count_pipes() as f32) / 2.0).round() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_steps_to_furthest_pipe_in_loop() {
        let input = ".....\n.S-7.\n.|.|.\n.L-J.\n.....";
        assert_eq!(find_farthest_connected_pipe(input), 4);

        let input = "..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...";
        assert_eq!(find_farthest_connected_pipe(input), 8);
    }
}
