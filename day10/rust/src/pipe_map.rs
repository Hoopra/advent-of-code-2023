use crate::model::{Pipe, PipeGrid, Position, SurroundingTiles};

// returns (map, starting_position)
pub fn construct_pipe_map(input: &str) -> (PipeGrid, Position) {
    let mut start_x = 0;
    let mut start_y = 0;

    let mut map: PipeGrid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, character)| {
                    if character == '.' {
                        return None;
                    }

                    if character == 'S' {
                        start_x = x;
                        start_y = y;
                    }

                    Some(Pipe::new((x, y), character))
                })
                .collect()
        })
        .collect();

    map[start_y] = map[start_y]
        .clone()
        .iter()
        .enumerate()
        .map(|(index, pipe)| {
            if index == start_x {
                return Some(find_starting_pipe(&map, (start_x, start_y)));
            }

            pipe.clone()
        })
        .collect();

    (map, (start_x, start_y))
}

fn find_starting_pipe(map: &PipeGrid, position: Position) -> Pipe {
    let SurroundingTiles {
        left,
        right,
        up,
        down,
    } = find_surrounding_tiles(map, position);

    Pipe::from_connections(
        position,
        left.is_some() && left.as_ref().unwrap().connects_right(),
        right.is_some() && right.as_ref().unwrap().connects_left(),
        up.is_some() && up.as_ref().unwrap().connects_down(),
        down.is_some() && down.as_ref().unwrap().connects_up(),
    )
}

fn find_surrounding_tiles(map: &PipeGrid, position: Position) -> SurroundingTiles {
    let (x, y) = position;

    let is_first_row = y == 0;
    let is_first_col = x == 0;

    SurroundingTiles {
        left: match is_first_row {
            true => &None,
            false => find_tile_in_map(map, (x, y - 1)),
        },
        right: find_tile_in_map(map, (x, y + 1)),
        up: match is_first_col {
            true => &None,
            false => find_tile_in_map(map, (x - 1, y)),
        },
        down: find_tile_in_map(map, (x + 1, y)),
    }
}

pub fn find_tile_in_map(map: &PipeGrid, position: Position) -> &Option<Pipe> {
    let (target_x, target_y) = position;

    let row = map.get(target_y);
    if row.is_none() {
        return &None;
    }

    let row = row.unwrap();

    row.get(target_x).unwrap_or(&None)
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
            &Some(Pipe::from_connections((1, 1), false, true, false, true))
        );
    }

    #[test]
    fn finds_correct_tile_in_map() {
        let input = "...........\n.S-------7.\n.|F-----7|.\n.||.....||.\n.||.....||.\n.|L-7.F-J|.\n.|..|.|..|.\n.L--J.L--J.\n...........";

        let (map, _) = construct_pipe_map(&input);

        assert_eq!(find_tile_in_map(&map, (2, 6)), &None);
    }
}
