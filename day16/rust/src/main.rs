mod energized;
mod model;

use energized::find_energized_tiles_from_beam;
use model::{Beam, Map};

use crate::energized::find_energized_tiles_from_position;
use std::fs::read_to_string;

fn main() {
    solve_part_1("../input.txt");
    solve_part_2("../input.txt");
}

fn solve_part_1(file_path: &str) {
    let text = read_to_string(file_path).unwrap();

    let energized = find_energized_tiles_from_position(&text, (0, 0));
    assert_eq!(energized.len(), 8249);
}

fn solve_part_2(file_path: &str) {
    let text = read_to_string(file_path).unwrap();

    let result = find_most_energized_beams_by_entry(&text);
    assert_eq!(result, 8444);
}

fn find_most_energized_beams_by_entry(text: &str) -> usize {
    let map = Map::from_text(&text);
    let beams = find_possible_starting_beams(&map);

    beams
        .iter()
        .map(|beam| {
            let energized = find_energized_tiles_from_beam(&map, &beam);
            energized.len()
        })
        .max()
        .unwrap()
}

fn find_possible_starting_beams(map: &Map) -> Vec<Beam> {
    let mut beams = vec![];

    let (x_max, y_max) = map.size;

    // top and bottom sides
    for x in 0..x_max {
        beams.push(Beam::new((x, 0), model::TravelDirection::S));
        beams.push(Beam::new((x, y_max - 1), model::TravelDirection::N));
    }

    // left and right sides
    for y in 0..y_max {
        beams.push(Beam::new((0, y), model::TravelDirection::E));
        beams.push(Beam::new((x_max - 1, y), model::TravelDirection::W));
    }

    beams
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_most_energized_tiles() {
        let input = ".|...\\....\n|.-.\\.....\n.....|-...\n........|.\n..........\n.........\\\n..../.\\\\..\n.-.-/..|..\n.|....-|.\\\n..//.|....";

        let energized = find_most_energized_beams_by_entry(input);
        assert_eq!(energized, 51);
    }

    #[test]
    fn finds_all_possible_beam_entries() {
        let input = ".|...\\....\n|.-.\\.....\n.....|-...\n........|.\n..........\n.........\\\n..../.\\\\..\n.-.-/..|..\n.|....-|.\\\n..//.|....";
        let map = Map::from_text(input);

        let beams = find_possible_starting_beams(&map);

        assert_eq!(beams.len(), 40);

        assert_eq!(
            beams.contains(&Beam::new((0, 9), model::TravelDirection::N)),
            true
        );

        assert_eq!(
            beams.contains(&Beam::new((0, 0), model::TravelDirection::S)),
            true
        );

        assert_eq!(
            beams.contains(&Beam::new((9, 0), model::TravelDirection::W)),
            true
        );
    }
}
