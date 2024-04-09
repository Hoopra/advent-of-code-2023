#[derive(PartialEq, Debug)]
pub enum Instruction {
    Remove,
    AddOrReplace(usize),
}

impl Instruction {
    pub fn from_text(text: &str) -> (String, Instruction) {
        let instruction_index = text.find(|value| value == '=' || value == '-').unwrap();

        let (label, focal_length) = text.split_at(instruction_index);

        let label = label.to_string();

        let focal_length = focal_length
            .chars()
            .filter(|value| value.is_ascii_digit())
            .collect::<String>()
            .parse();

        let instruction = match text
            .chars()
            .collect::<Vec<char>>()
            .get(instruction_index)
            .unwrap()
        {
            '=' => (label, Instruction::AddOrReplace(focal_length.unwrap())),
            _ => (label, Instruction::Remove),
        };
        instruction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_instruction_set() {
        let input = "rx=5";
        let instruction = Instruction::from_text(input);

        assert_eq!(
            instruction,
            (String::from("rx"), Instruction::AddOrReplace(5))
        );

        let input = "hh-";
        let instruction = Instruction::from_text(input);

        assert_eq!(instruction, (String::from("hh"), Instruction::Remove));
    }
}
