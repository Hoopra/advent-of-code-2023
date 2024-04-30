mod pathing;

use pathing::{find_shortest_path_in_graph, Graph};
use std::fs::read_to_string;

fn main() {
    let text = read_to_string("input.txt").unwrap();

    solve_part_1(&text);
    solve_part_2(&text);
}

fn solve_part_1(text: &str) {
    let graph = Graph::from_text(&text);

    let result = find_shortest_path_in_graph(&graph, (0, 0), graph.size, 1, 3);

    assert_eq!(result, Some(686));
}

fn solve_part_2(text: &str) {
    let graph = Graph::from_text(&text);

    let result = find_shortest_path_in_graph(&graph, (0, 0), graph.size, 4, 10);

    assert_eq!(result, Some(801));
}
