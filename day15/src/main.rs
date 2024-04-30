mod hash;
mod instruction_set;
mod lens;

use std::{collections::HashMap, fs::read_to_string};

use instruction_set::Instruction;
use lens::Lens;

use crate::hash::{compute_hash_value, compute_hash_values};

fn main() {
    solve_part_1("input.txt");
    solve_part_2("input.txt");
}

fn solve_part_1(file_path: &str) {
    let text = read_to_string(file_path).unwrap();

    let result = compute_hash_values(&text);
    assert_eq!(result, 516657);
}

fn solve_part_2(file_path: &str) {
    let text = read_to_string(file_path).unwrap();

    let map = parse_instructions(&text);
    let result = sum_focal_map(&map);

    assert_eq!(result, 210906);
}

fn parse_instructions(text: &str) -> HashMap<usize, Vec<Lens>> {
    let mut map: HashMap<usize, Vec<Lens>> = HashMap::new();

    text.split(',').for_each(|text| {
        let (label, instruction) = Instruction::from_text(text);

        let room_number = compute_hash_value(&label);

        let mut lenses = map.get(&room_number).unwrap_or(&vec![]).to_vec();

        match instruction {
            Instruction::AddOrReplace(focal_length) => {
                add_or_replace_lens(&mut lenses, (label, focal_length));
            }
            Instruction::Remove => {
                lenses = lenses
                    .iter()
                    .filter_map(|lens| match lens.0 != label {
                        true => Some(lens.clone()),
                        false => None,
                    })
                    .collect()
            }
        }

        map.insert(room_number, lenses.to_owned());
    });

    map
}

fn sum_focal_map(map: &HashMap<usize, Vec<Lens>>) -> usize {
    map.iter().fold(0, |previous, (room, lenses)| {
        let focals = lenses.iter().enumerate().fold(0, |sum, (index, lens)| {
            let (_, focal_length) = lens;

            sum + (*room + 1) * (index + 1) * (*focal_length)
        });

        previous + focals
    })
}

pub fn add_or_replace_lens(lenses: &mut Vec<Lens>, lens: Lens) -> &Vec<Lens> {
    let (label, focal_length) = &lens;

    let exists = lenses.iter().position(|lens| &lens.0 == label);

    match exists {
        Some(index) => lenses[index] = (label.to_string(), *focal_length),
        _ => lenses.push(lens),
    }

    lenses
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_instructions() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        let map = parse_instructions(input);

        assert_eq!(
            map.get(&0).unwrap(),
            &vec![(String::from("rn"), 1), (String::from("cm"), 2)]
        );

        assert_eq!(map.get(&1), Some(&vec![]));
        assert_eq!(map.get(&2), None);

        assert_eq!(
            map.get(&3).unwrap(),
            &vec![
                (String::from("ot"), 7),
                (String::from("ab"), 5),
                (String::from("pc"), 6)
            ]
        );

        let result = sum_focal_map(&map);

        assert_eq!(result, 145);
    }
}
