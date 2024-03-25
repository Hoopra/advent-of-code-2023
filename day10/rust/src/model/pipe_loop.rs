use std::collections::HashMap;

use super::{Pipe, PipeMap, Position};

pub struct PipeLoop {
    pipes: HashMap<Position, Pipe>,
}

impl PipeLoop {
    pub fn new(pipes: HashMap<Position, Pipe>) -> PipeLoop {
        PipeLoop { pipes }
    }

    pub fn from_map(map: &PipeMap) -> PipeLoop {
        let mut result: HashMap<Position, Pipe> = HashMap::new();

        let start = map.start;

        let mut previous: Option<Position> = None;
        let mut current = start;

        loop {
            let pipe = map.get_tile(&current);

            if pipe.is_none() {
                break;
            }

            let pipe = pipe.as_ref().unwrap();
            result.insert(current, **pipe);

            let next = find_connecting_pipe(pipe, previous);

            previous = Some(current);
            current = next;

            if current == start {
                break;
            }
        }

        PipeLoop::new(result)
    }
}

impl PipeLoop {
    pub fn count_pipes(&self) -> usize {
        self.pipes.len()
    }

    pub fn get(&self, position: &Position) -> Option<&Pipe> {
        self.pipes.get(position)
    }

    pub fn find_pipes_right(&self, position: &Position) -> Vec<char> {
        let (x_start, y) = position;

        self.pipes
            .iter()
            .filter_map(|(position, pipe)| {
                let (pipe_x, pipe_y) = position;

                match pipe_x > x_start && pipe_y == y {
                    true => Some(pipe.symbol),
                    _ => None,
                }
            })
            .collect()
    }

    #[cfg(test)]
    pub fn draw_in_map(&self, map: &PipeMap, marks: Vec<Position>) {
        let (x_max, y_max) = map.size;

        println!("");
        for y in 0..y_max {
            for x in 0..x_max {
                let pipe = self.get(&(x, y));

                match pipe {
                    None => match marks.contains(&(x, y)) {
                        true => {
                            print!("X")
                        }
                        false => print!(" "),
                    },
                    Some(pipe) => print!("{}", pipe.symbol),
                }
            }
            println!("");
        }
        println!("");
    }
}

fn find_connecting_pipe(pipe: &Pipe, previous: Option<Position>) -> Position {
    let connectors = pipe.get_connectors();

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

        let map = PipeMap::from_text(&input);
        let pipe_loop = PipeLoop::from_map(&map);

        pipe_loop.draw_in_map(&map, vec![]);

        assert_eq!(pipe_loop.count_pipes(), 8);
    }

    #[test]
    fn constructs_bigger_pipe_loop() {
        let input = "...........\n.S-------7.\n.|F-----7|.\n.||.....||.\n.||.....||.\n.|L-7.F-J|.\n.|..|.|..|.\n.L--J.L--J.\n...........";

        let map = PipeMap::from_text(&input);
        let pipe_loop = PipeLoop::from_map(&map);

        assert_eq!(&pipe_loop.count_pipes(), &46);

        assert_eq!(&pipe_loop.get(&(1, 1)).unwrap().symbol, &'F');
        assert_eq!(&pipe_loop.get(&(0, 1)).is_some(), &false);

        assert_eq!(&pipe_loop.get(&(1, 6)).unwrap().symbol, &'|');
        assert_eq!(&pipe_loop.get(&(4, 6)).unwrap().symbol, &'|');
        assert_eq!(&pipe_loop.get(&(6, 6)).unwrap().symbol, &'|');
        assert_eq!(&pipe_loop.get(&(9, 6)).unwrap().symbol, &'|');
    }
}
