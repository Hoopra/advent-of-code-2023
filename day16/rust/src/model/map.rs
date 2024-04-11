#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use super::{MapFeature, Position};

pub struct Map {
    tiles: HashMap<Position, MapFeature>,
    pub size: (usize, usize),
}

impl Map {
    pub fn from_text(text: &str) -> Map {
        let mut tiles = HashMap::new();
        let mut size = (0, 0);

        for (y, line) in text.lines().enumerate() {
            for (x, symbol) in line.chars().enumerate() {
                let feature = match symbol {
                    '|' => MapFeature::SplitterNS,
                    '-' => MapFeature::SplitterWE,
                    '\\' => MapFeature::MirrorSE,
                    '/' => MapFeature::MirrorSW,
                    _ => MapFeature::Empty,
                };

                tiles.insert((x, y), feature);
                size.0 = x + 1;
            }

            size.1 = y + 1;
        }

        Map { tiles, size }
    }
}

impl Map {
    pub fn get(&self, position: &Position) -> Option<&MapFeature> {
        self.tiles.get(position)
    }

    pub fn draw_energized(&self, energized: &HashSet<Position>) {
        let (x_max, y_max) = self.size;

        println!("");
        for y in 0..y_max {
            for x in 0..x_max {
                let position = (x, y);

                match energized.contains(&position) {
                    true => print!("#"),
                    false => print!("."),
                }
            }
            println!("");
        }
        println!("");
    }
}
