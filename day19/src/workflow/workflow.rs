use super::WorkflowStep;
use crate::part::{Part, PartCombination};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum WorkflowResult {
    Accept,
    Reject,
    Next(String),
}

impl WorkflowResult {
    pub fn from_text(text: &str) -> WorkflowResult {
        match text {
            "A" => WorkflowResult::Accept,
            "R" => WorkflowResult::Reject,
            value => WorkflowResult::Next(value.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct Workflow {
    id: String,
    steps: Vec<WorkflowStep>,
}

impl Workflow {
    pub fn from_text(text: &str) -> Workflow {
        let steps_start = text.find('{').unwrap();
        let id = text.chars().take(steps_start).collect();

        let steps_start = steps_start + 1;
        let steps_text: String = text
            .chars()
            .skip(steps_start)
            .take(text.len() - steps_start - 1)
            .collect();

        let steps = steps_text
            .split(',')
            .map(|text| WorkflowStep::from_text(text))
            .collect();

        Workflow { id, steps }
    }
}

impl Workflow {
    pub fn apply_to_part(&self, part: &Part) -> WorkflowResult {
        for step in &self.steps {
            let result = step.apply_to_part(part);

            if result.is_some() {
                return result.unwrap();
            }
        }

        self.steps.last().unwrap().target.clone()
    }

    pub fn find_accepted_combinations(&self, part: &PartCombination) -> Vec<PartCombination> {
        let mut result = vec![];

        for i in 0..self.steps.len() {
            let mut part = part.clone();

            for j in 0..=i {
                let step = self.steps.get(j).unwrap();

                part = match j == i {
                    true => step.narrow_by_comparison(&part),
                    _ => step.narrow_by_reverse_comparison(&part),
                }
            }

            if part.is_valid() {
                result.push(part);
            }
        }

        result
    }
}

pub fn parse_workflows(text: &str) -> HashMap<String, Workflow> {
    let mut result = HashMap::new();

    text.lines().for_each(|inner| {
        let workflow = Workflow::from_text(inner);
        result.insert(workflow.id.to_string(), workflow);
    });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_workflow_from_text() {
        let workflow = Workflow::from_text("dcz{a<2168:czk,a>3385:xnc,x>889:gpr,nmv}");

        assert_eq!(workflow.id, "dcz");
        assert_eq!(workflow.steps.len(), 4);
    }

    #[test]
    fn finds_accepted_combinations() {
        let workflow = Workflow::from_text("dcz{a<2168:czk,a>3385:xnc,x>889:gpr,nmv}");

        let part = PartCombination::new();
        let result = workflow.find_accepted_combinations(&part);

        assert_eq!(result.len(), 4);

        assert_eq!(
            result.get(0).unwrap(),
            &PartCombination::with_values(
                (1, 2167),
                (1, 4000),
                (1, 4000),
                (1, 4000),
                WorkflowResult::Next(String::from("czk"))
            ),
        );

        assert_eq!(
            result.get(1).unwrap(),
            &PartCombination::with_values(
                (3386, 4000),
                (1, 4000),
                (1, 4000),
                (1, 4000),
                WorkflowResult::Next(String::from("xnc"))
            ),
        );

        assert_eq!(
            result.get(2).unwrap(),
            &PartCombination::with_values(
                (2168, 3385),
                (890, 4000),
                (1, 4000),
                (1, 4000),
                WorkflowResult::Next(String::from("gpr"))
            )
        );

        assert_eq!(
            result.get(3).unwrap(),
            &PartCombination::with_values(
                (2168, 3385),
                (1, 889),
                (1, 4000),
                (1, 4000),
                WorkflowResult::Next(String::from("nmv"))
            )
        );
    }

    #[test]
    fn finds_accepted_combinations_other_workflow() {
        let workflow = Workflow::from_text("px{a<2006:qkq,m>2090:A,rfg}");

        let part = PartCombination::new();
        let result = workflow.find_accepted_combinations(&part);

        assert_eq!(result.len(), 3);

        assert_eq!(
            result.get(0).unwrap(),
            &PartCombination::with_values(
                (1, 2005),
                (1, 4000),
                (1, 4000),
                (1, 4000),
                WorkflowResult::Next(String::from("qkq"))
            ),
        );

        assert_eq!(
            result.get(1).unwrap(),
            &PartCombination::with_values(
                (2006, 4000),
                (1, 4000),
                (2091, 4000),
                (1, 4000),
                WorkflowResult::Accept
            ),
        );

        assert_eq!(
            result.get(2).unwrap(),
            &PartCombination::with_values(
                (2006, 4000),
                (1, 4000),
                (1, 2090),
                (1, 4000),
                WorkflowResult::Next(String::from("rfg"))
            )
        );
    }
}
