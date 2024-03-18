use crate::pipe::{Pipe, PipeGrid, PipePart, Position};
use crate::pipe_map::{construct_pipe_map, find_tile_in_map};

pub fn find_farthest_connected_pipe(input: &str) -> u32 {
    let (map, start) = construct_pipe_map(input);
    let pipe_loop = construct_pipe_loop(&map, start);

    ((pipe_loop.len() as f32) / 2.0).round() as u32
}

fn construct_pipe_loop(map: &PipeGrid, start: Position) -> Vec<Position> {
    let mut result = vec![];

    let mut previous: Option<Position> = None;
    let mut current = start;

    loop {
        result.push(current);

        let pipe = find_tile_in_map(map, current);

        if pipe.is_none() {
            break;
        }

        let pipe = pipe.unwrap();

        let next = find_connecting_pipe(pipe, current, previous);

        previous = Some(current);
        current = next;

        if current == start {
            break;
        }
    }

    result
}

fn find_connecting_pipe(pipe: Pipe, position: Position, previous: Option<Position>) -> Position {
    let Pipe {
        left,
        right,
        up,
        down,
    } = pipe;

    let (row, col) = position;

    let mut connectors: Vec<Position> = vec![];
    if left == PipePart::Connection {
        connectors.push((row, col - 1));
    }

    if right == PipePart::Connection {
        connectors.push((row, col + 1));
    }

    if up == PipePart::Connection {
        connectors.push((row - 1, col));
    }

    if down == PipePart::Connection {
        connectors.push((row + 1, col));
    }

    if previous.is_none() {
        return *connectors.get(0).unwrap();
    }

    *connectors
        .into_iter()
        .filter(|position| position != &previous.unwrap())
        .collect::<Vec<Position>>()
        .get(0)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_pipe_loop() {
        let input = ".....\n.S-7.\n.|.|.\n.L-J.\n.....";

        let (map, start) = construct_pipe_map(&input);
        let pipe_loop = construct_pipe_loop(&map, start);

        assert_eq!(pipe_loop.len(), 8);
    }

    #[test]
    fn finds_steps_to_furthest_pipe_in_loop() {
        let input = ".....\n.S-7.\n.|.|.\n.L-J.\n.....";

        assert_eq!(find_farthest_connected_pipe(input), 4);

        let input = "..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...";
        assert_eq!(find_farthest_connected_pipe(input), 8);
    }
}
