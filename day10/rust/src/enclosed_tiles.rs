use crate::model::{Direction, PipeLoop, PipeMap, Position};

pub fn find_enclosed_tiles_in_map(input: &str) -> Vec<Position> {
    let map = PipeMap::from_text(input);

    let pipe_loop = PipeLoop::from_map(&map);
    let empty_tiles = find_empty_tiles(&map, &pipe_loop);

    find_enclosed_tiles(&empty_tiles, &pipe_loop)
}

fn find_empty_tiles(map: &PipeMap, pipe_loop: &PipeLoop) -> Vec<Position> {
    let (x_max, y_max) = map.size;

    let mut result = vec![];

    for x in 0..x_max {
        for y in 0..y_max {
            let position = (x, y);
            let tile = pipe_loop.get(&position);

            if tile.is_none() {
                result.push(position)
            }
        }
    }

    result
}

fn find_enclosed_tiles(empty_tiles: &Vec<Position>, pipe_loop: &PipeLoop) -> Vec<Position> {
    empty_tiles
        .iter()
        .filter(|tile_position| is_tile_enclosed(tile_position, pipe_loop))
        .map(|position| position.clone())
        .collect()
}

fn is_tile_enclosed(tile: &Position, pipe_loop: &PipeLoop) -> bool {
    let enclosement_scores = vec![
        is_tile_enclosed_direction(tile, pipe_loop, Direction::Left),
        is_tile_enclosed_direction(tile, pipe_loop, Direction::Right),
        is_tile_enclosed_direction(tile, pipe_loop, Direction::Up),
        is_tile_enclosed_direction(tile, pipe_loop, Direction::Down),
    ];

    enclosement_scores.iter().any(|enclosed| enclosed == &true)
}

fn is_tile_enclosed_direction(tile: &Position, pipe_loop: &PipeLoop, direction: Direction) -> bool {
    let pipes: Vec<char> = pipe_loop.find_pipes_in_direction(tile, &direction);

    let excluded_symbol = match direction {
        Direction::Left | Direction::Right => '-',
        _ => '|',
    };

    is_tile_enclosed_by_pipes(pipes, excluded_symbol)
}

fn is_tile_enclosed_by_pipes(pipes: Vec<char>, excluded_symbol: char) -> bool {
    let mut vertical_pipes: f32 = 0.0;
    let mut bends_1: f32 = 0.0;
    let mut bends_2: f32 = 0.0;

    pipes.iter().for_each(|symbol| match *symbol {
        'L' | '7' => bends_1 += 0.5,
        'F' | 'J' => bends_2 += 0.5,
        symbol if symbol == excluded_symbol => {}
        _ => vertical_pipes += 1.0,
    });

    let pipe_score = vertical_pipes + bends_1 - bends_2;

    pipe_score % 2.0 != 0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determines_if_tile_enclosed() {
        let input = "...........\n.S-------7.\n.|F-----7|.\n.||.....||.\n.||.....||.\n.|L-7.F-J|.\n.|..|.|..|.\n.L--J.L--J.\n...........";

        let map = PipeMap::from_text(&input);
        let pipe_loop = PipeLoop::from_map(&map);

        assert_eq!(is_tile_enclosed(&(0, 1), &pipe_loop), false);

        assert_eq!(is_tile_enclosed(&(3, 3), &pipe_loop), false);

        assert_eq!(is_tile_enclosed(&(3, 6), &pipe_loop), true);

        assert_eq!(is_tile_enclosed(&(5, 5), &pipe_loop), false);

        assert_eq!(is_tile_enclosed(&(5, 7), &pipe_loop), false);
    }

    #[test]
    fn finds_empty_tiles() {
        let input = "FF7FSF7F7F7F7F7F---7\nL|LJ||||||||||||F--J\nFL-7LJLJ||||||LJL-77\nF--JF--7||LJLJ7F7FJ-\nL---JF-JLJ.||-FJLJJ7\n|F|F-JF---7F7-L7L|7|\n|FFJF7L7F-JF7|JL---7\n7-L-JL7||F7|L7F-7F7|\nL.L7LFJ|||||FJL7||LJ\nL7JLJL-JLJLJL--JLJ.L";

        let map = PipeMap::from_text(&input);
        let pipe_loop = PipeLoop::from_map(&map);

        assert_eq!(find_empty_tiles(&map, &pipe_loop).len(), 40);
    }

    #[test]
    fn finds_enclosed_tiles_small_map() {
        let input = "...........\n.S-------7.\n.|F-----7|.\n.||.....||.\n.||.....||.\n.|L-7.F-J|.\n.|..|.|..|.\n.L--J.L--J.\n...........";

        let tiles = find_enclosed_tiles_in_map(&input);
        assert_eq!(tiles, vec![(2, 6), (3, 6), (7, 6), (8, 6)]);
    }

    #[test]
    fn finds_enclosed_tiles_large_map() {
        let input = ".F----7F7F7F7F-7....\n.|F--7||||||||FJ....\n.||.FJ||||||||L7....\nFJL7L7LJLJ||LJ.L-7..\nL--J.L7...LJS7F-7L7.\n....F-J..F7FJ|L7L7L7\n....L7.F7||L7|.L7L7|\n.....|FJLJ|FJ|F7|.LJ\n....FJL-7.||.||||...\n....L---J.LJ.LJLJ...";

        assert_eq!(find_enclosed_tiles_in_map(input).len(), 8);

        let map = PipeMap::from_text(input);
        let pipe_loop = PipeLoop::from_map(&map);

        pipe_loop.draw_in_map(&map, vec![]);

        assert_eq!(pipe_loop.count_pipes(), 140);
        assert_eq!(pipe_loop.get(&(15, 3)).is_some(), true);

        assert_eq!(is_tile_enclosed(&(0, 0), &pipe_loop), false);
        assert_eq!(is_tile_enclosed(&(12, 8), &pipe_loop), false);

        assert_eq!(is_tile_enclosed(&(14, 3), &pipe_loop), true);
        assert_eq!(is_tile_enclosed(&(7, 4), &pipe_loop), true);
        assert_eq!(is_tile_enclosed(&(8, 4), &pipe_loop), true);
        assert_eq!(is_tile_enclosed(&(9, 4), &pipe_loop), true);

        assert_eq!(is_tile_enclosed(&(7, 5), &pipe_loop), true);
        assert_eq!(is_tile_enclosed(&(8, 5), &pipe_loop), true);
        assert_eq!(is_tile_enclosed(&(6, 6), &pipe_loop), true);
        assert_eq!(is_tile_enclosed(&(14, 6), &pipe_loop), true);
    }

    #[test]
    fn finds_enclosed_tiles_larger_map() {
        let input = "FF7FSF7F7F7F7F7F---7\nL|LJ||||||||||||F--J\nFL-7LJLJ||||||LJL-77\nF--JF--7||LJLJ7F7FJ-\nL---JF-JLJ.||-FJLJJ7\n|F|F-JF---7F7-L7L|7|\n|FFJF7L7F-JF7|JL---7\n7-L-JL7||F7|L7F-7F7|\nL.L7LFJ|||||FJL7||LJ\nL7JLJL-JLJLJL--JLJ.L";

        assert_eq!(find_enclosed_tiles_in_map(input).len(), 10);

        let map = PipeMap::from_text(input);
        let pipe_loop = PipeLoop::from_map(&map);

        let locations = vec![(10, 4), (11, 5), (13, 6)];
        pipe_loop.draw_in_map(&map, locations);

        assert_eq!(pipe_loop.count_pipes(), 160);
        assert_eq!(pipe_loop.get(&(15, 3)).is_some(), true);

        assert_eq!(is_tile_enclosed(&(2, 5), &pipe_loop), false);
        assert_eq!(is_tile_enclosed(&(14, 3), &pipe_loop), true);

        assert_eq!(is_tile_enclosed(&(10, 4), &pipe_loop), true);
        assert_eq!(is_tile_enclosed(&(11, 4), &pipe_loop), true);
        assert_eq!(is_tile_enclosed(&(12, 4), &pipe_loop), true);
        assert_eq!(is_tile_enclosed(&(13, 4), &pipe_loop), true);

        assert_eq!(is_tile_enclosed(&(11, 5), &pipe_loop), true);
        assert_eq!(is_tile_enclosed(&(12, 5), &pipe_loop), true);
        assert_eq!(is_tile_enclosed(&(13, 5), &pipe_loop), true);

        assert_eq!(is_tile_enclosed(&(13, 6), &pipe_loop), true);
        assert_eq!(is_tile_enclosed(&(14, 6), &pipe_loop), true);
    }
}
