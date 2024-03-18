use crate::pipe::{Pipe, PipeGrid, PipePart, Position, SurroundingTiles};

// returns (map, starting_position)
pub fn construct_pipe_map(input: &str) -> (PipeGrid, Position) {
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

                    Some(Pipe::from_connections(
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

    Pipe::from_connections(
        left.is_some() && left.unwrap().right == PipePart::Connection,
        right.is_some() && right.unwrap().left == PipePart::Connection,
        up.is_some() && up.unwrap().down == PipePart::Connection,
        down.is_some() && down.unwrap().up == PipePart::Connection,
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

pub fn find_tile_in_map(map: &PipeGrid, position: Position) -> &Option<Pipe> {
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

        let (map, start) = construct_pipe_map(&input);

        assert_eq!(start, (1, 1));

        assert_eq!(map.get(0).unwrap(), &vec![None, None, None, None, None]);
        assert_eq!(
            map.get(1).unwrap().get(1).unwrap(),
            &Some(Pipe::from_connections(false, true, false, true))
        );
    }
}
