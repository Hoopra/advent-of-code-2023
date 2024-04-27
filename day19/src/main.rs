mod constants;
mod part;
mod workflow;

use part::{parse_parts, PartCombination};
use std::fs::read_to_string;
use workflow::{parse_workflows, WorkflowResult};

fn main() {
    let input = read_to_string("../input.txt").unwrap();

    solve_part_1(&input);
    solve_part_2(&input);
}

fn solve_part_1(text: &str) {
    let accepted_rating = find_accepted_component_rating(text);

    assert_eq!(accepted_rating, 397134);
}

fn find_accepted_component_rating(text: &str) -> u32 {
    let components: Vec<&str> = text.split("\n\n").collect();

    let workflows = parse_workflows(components.get(0).unwrap());

    let parts = parse_parts(components.get(1).unwrap());

    let first_workflow = workflows.get("in").unwrap();

    parts
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
        .sum()
}

fn solve_part_2(text: &str) {
    let result = find_accepted_combinations(text);

    assert_eq!(result, 127517902575337);
}

fn find_accepted_combinations(text: &str) -> u64 {
    let components: Vec<&str> = text.split("\n\n").collect();

    let workflows = parse_workflows(components.get(0).unwrap());

    let mut accepted_parts = vec![];
    let mut parts = vec![PartCombination::new()];

    while parts.len() > 0 {
        let next_part = parts.pop().unwrap();

        match &next_part.destination {
            WorkflowResult::Accept => accepted_parts.push(next_part),
            WorkflowResult::Reject => {}
            WorkflowResult::Next(id) => {
                let workflow = workflows.get(id).unwrap();
                parts.append(&mut workflow.find_accepted_combinations(&next_part))
            }
        }
    }

    accepted_parts
        .iter()
        .map(|part| part.get_combinations())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_accepted_component_rating() {
        let text= "px{a<2006:qkq,m>2090:A,rfg}\npv{a>1716:R,A}\nlnx{m>1548:A,A}\nrfg{s<537:gd,x>2440:R,A}\nqs{s>3448:A,lnx}\nqkq{x<1416:A,crn}\ncrn{x>2662:A,R}\nin{s<1351:px,qqz}\nqqz{s>2770:qs,m<1801:hdj,R}\ngd{a>3333:R,R}\nhdj{m>838:A,pv}\n\n{x=787,m=2655,a=1222,s=2876}\n{x=1679,m=44,a=2067,s=496}\n{x=2036,m=264,a=79,s=2244}\n{x=2461,m=1339,a=466,s=291}\n{x=2127,m=1623,a=2188,s=1013}";

        let result = find_accepted_component_rating(text);

        assert_eq!(result, 19114);
    }

    #[test]
    fn finds_possible_accepted_rating() {
        let text= "px{a<2006:qkq,m>2090:A,rfg}\npv{a>1716:R,A}\nlnx{m>1548:A,A}\nrfg{s<537:gd,x>2440:R,A}\nqs{s>3448:A,lnx}\nqkq{x<1416:A,crn}\ncrn{x>2662:A,R}\nin{s<1351:px,qqz}\nqqz{s>2770:qs,m<1801:hdj,R}\ngd{a>3333:R,R}\nhdj{m>838:A,pv}";

        let result = find_accepted_combinations(text);

        assert_eq!(result, 167409079868000);
    }
}
