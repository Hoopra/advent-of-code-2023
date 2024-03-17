use std::fs::read_to_string;

type PipeGrid = Vec<Vec<Option<Pipe>>>;

type Position = (usize, usize);

#[derive(Debug, PartialEq, Clone, Copy)]
struct Pipe {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

struct SurroundingTiles<'a> {
    left: &'a Option<Pipe>,
    right: &'a Option<Pipe>,
    up: &'a Option<Pipe>,
    down: &'a Option<Pipe>,
}

impl Pipe {
    pub fn new(left: bool, right: bool, up: bool, down: bool) -> Pipe {
        Pipe {
            left,
            right,
            up,
            down,
        }
    }
}

fn main() {
    let file_path = "../input.txt";
    let text = read_to_string(file_path).unwrap();

    let most_steps = find_farthest_connected_pipe(&text);
    assert_eq!(most_steps, 6923);
}

fn find_farthest_connected_pipe(input: &str) -> u32 {
    let (map, start) = construct_map(input);
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
        println!("{:?}", pipe);

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
    if left {
        connectors.push((row, col - 1));
    }

    if right {
        connectors.push((row, col + 1));
    }

    if up {
        connectors.push((row - 1, col));
    }

    if down {
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

// returns (map, starting_position)
fn construct_map(input: &str) -> (PipeGrid, Position) {
    let mut start_row = 0;
    let mut start_col = 0;

    let mut map: PipeGrid = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, character)| {
                    if character == '.' {
                        return None;
                    }

                    if character == 'S' {
                        start_row = row;
                        start_col = col;
                    }

                    Some(Pipe::new(
                        character == 'J' || character == '7' || character == '-',
                        character == 'L' || character == 'F' || character == '-',
                        character == '|' || character == 'J' || character == 'L',
                        character == '|' || character == 'F' || character == '7',
                    ))
                })
                .collect()
        })
        .collect();

    map[start_row] = map[start_row]
        .clone()
        .iter()
        .enumerate()
        .map(|(index, position)| {
            if index == start_col {
                return Some(find_starting_pipe(&map, (start_row, start_col)));
            }

            *position
        })
        .collect();

    (map, (start_row, start_col))
}

fn find_starting_pipe(map: &PipeGrid, position: Position) -> Pipe {
    let SurroundingTiles {
        left,
        right,
        up,
        down,
    } = find_surrounding_tiles(map, position);

    Pipe::new(
        left.is_some() && left.unwrap().right,
        right.is_some() && right.unwrap().left,
        up.is_some() && up.unwrap().down,
        down.is_some() && down.unwrap().up,
    )
}

fn find_surrounding_tiles(map: &PipeGrid, position: Position) -> SurroundingTiles {
    let (row, col) = position;

    let is_first_row = row == 0;
    let is_first_col = col == 0;

    SurroundingTiles {
        left: match is_first_col {
            true => &None,
            false => find_tile_in_map(map, (row, col - 1)),
        },
        right: find_tile_in_map(map, (row, col + 1)),
        up: match is_first_row {
            true => &None,
            false => find_tile_in_map(map, (row - 1, col)),
        },
        down: find_tile_in_map(map, (row + 1, col)),
    }
}

fn find_tile_in_map(map: &PipeGrid, position: Position) -> &Option<Pipe> {
    let (target_row, target_col) = position;

    let row = map.get(target_row);
    if row.is_none() {
        return &None;
    }

    let row = row.unwrap();
    row.get(target_col).unwrap_or(&None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_map_from_text() {
        let input = ".....\n.S-7.\n.|.|.\n.L-J.\n.....";

        let (map, start) = construct_map(&input);

        assert_eq!(start, (1, 1));

        assert_eq!(map.get(0).unwrap(), &vec![None, None, None, None, None]);
        assert_eq!(
            map.get(1).unwrap().get(1).unwrap(),
            &Some(Pipe::new(false, true, false, true))
        );
    }

    #[test]
    fn constructs_pipe_loop() {
        let input = ".....\n.S-7.\n.|.|.\n.L-J.\n.....";

        let (map, start) = construct_map(&input);
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
