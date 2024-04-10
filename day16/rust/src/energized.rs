use crate::model::MapFeature;
use crate::model::{Beam, Map, Position};
use std::collections::HashSet;

pub fn find_energized_tiles(text: &str, initial_position: Position) -> HashSet<Position> {
    let map = Map::from_text(&text);

    let mut cache: HashSet<String> = HashSet::new();
    let mut energized_tiles: HashSet<Position> = HashSet::from([initial_position]);
    let mut beams: Vec<Beam> = vec![Beam::default(initial_position)];

    while beams.len() > 0 {
        let mut beam = beams.pop().unwrap();

        loop {
            let position = &beam.position;

            if position.is_none() {
                break;
            }

            let key = beam.get_key();
            let cache_hit = cache.get(&key);
            if cache_hit.is_some() {
                break;
            }

            cache.insert(key);

            let position = position.unwrap();
            let tile = map.get(&position);

            if tile.is_none() {
                break;
            }

            energized_tiles.insert(position);

            let tile = tile.unwrap();

            match tile {
                MapFeature::SplitterWE | MapFeature::SplitterNS => {
                    let mut new_beams = beam.split(tile);
                    beams.append(&mut new_beams);
                    break;
                }
                MapFeature::MirrorSW | MapFeature::MirrorSE => beam.update_direction(&tile),
                _ => {}
            }

            beam.update_position();
        }
    }

    energized_tiles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_energized_tiles_simple() {
        let input = ".............\n..........|..\n.............";

        let energized = find_energized_tiles(input, (0, 1));
        assert_eq!(energized.len(), 13);

        let input = "...\\...\n.......\n.......\n.......\n.......\n...-../\n.......";

        let energized = find_energized_tiles(input, (0, 0));
        assert_eq!(energized.len(), 20);
    }

    #[test]
    fn calculates_energized_tiles_all_features() {
        let input = ".......|........-....\\\n......................\n......................\n/.....................\n.....................|\n......................\n......................\n\\......-......\\...././\n......................\n......................\n......................\n......................\n..............\\.-../..\n......................";

        let energized = find_energized_tiles(input, (0, 0));
        assert_eq!(energized.len(), 89);
    }

    #[test]
    fn calculates_energized_tiles_all_tiles() {
        let input = "...................\\\n/...............\\/\\.\n./.............\\....\n../...........\\.....\n...|.........-......\n....|.......-.......\n...-........|.......\n..\\........../......\n.\\............/.....\n\\............../\\/\\/";

        let energized = find_energized_tiles(input, (0, 0));
        assert_eq!(energized.len(), 200);
    }

    #[test]
    fn calculates_energized_tiles_mirrors() {
        let input = ".........|..........\n....................\n....................\n.............-......\n....-...............\n....................\n....................\n....................\n....|....-...|......\n....................\n....................";

        let energized = find_energized_tiles(input, (0, 0));
        assert_eq!(energized.len(), 75);
    }

    #[test]
    fn calculates_energized_tiles_sample() {
        let input = ".|...\\....\n|.-.\\.....\n.....|-...\n........|.\n..........\n.........\\\n..../.\\\\..\n.-.-/..|..\n.|....-|.\\\n..//.|....";

        let energized = find_energized_tiles(input, (0, 0));

        assert_eq!(energized.contains(&(1, 1)), true);
        assert_eq!(energized.contains(&(9, 2)), true);
        assert_eq!(energized.contains(&(7, 6)), true);
        assert_eq!(energized.contains(&(3, 8)), true);
        assert_eq!(energized.contains(&(5, 9)), true);

        assert_eq!(energized.len(), 46);
    }
}
