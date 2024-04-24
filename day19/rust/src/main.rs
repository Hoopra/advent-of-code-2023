mod part;
mod workflow;

use part::parse_parts;
use std::fs::read_to_string;
use workflow::{parse_workflows, WorkflowResult};

fn main() {
    let input = read_to_string("../input.txt").unwrap();

    solve_part_1(&input);
}

fn solve_part_1(text: &str) {
    let components: Vec<&str> = text.split("\n\n").collect();

    let workflows = parse_workflows(components.get(0).unwrap());

    let parts = parse_parts(components.get(1).unwrap());

    let first_workflow = workflows.get("in").unwrap();

    let accepted_rating: u32 = parts
        .iter()
        .map(|part| {
            let mut workflow = first_workflow;

            loop {
                let result = workflow.apply_to_part(part);
                match result {
                    WorkflowResult::Accept => return part.get_rating(),
                    WorkflowResult::Reject => return 0,
                    WorkflowResult::Next(id) => {
                        workflow = workflows.get(&id).unwrap();
                        continue;
                    }
                }
            }
        })
        .sum();

    assert_eq!(accepted_rating, 397134);
}
