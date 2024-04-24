use std::collections::HashMap;

use crate::part::{Part, PartCharacteristic};

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

#[derive(Debug, PartialEq, Clone)]
pub enum ComparisonOperator {
    GreaterThan,
    LessThan,
}

impl ComparisonOperator {
    pub fn from_symbol(symbol: char) -> ComparisonOperator {
        match symbol {
            '>' => ComparisonOperator::GreaterThan,
            _ => ComparisonOperator::LessThan,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WorkflowComparison {
    lhs: PartCharacteristic,
    operator: ComparisonOperator,
    rhs: u32,
}

impl WorkflowComparison {
    pub fn from_text(text: &str) -> WorkflowComparison {
        let operator_index = text.find(|char| char == '>' || char == '<').unwrap();

        let operator = ComparisonOperator::from_symbol(text.chars().nth(operator_index).unwrap());

        let rhs = text
            .chars()
            .skip(operator_index + 1)
            .collect::<String>()
            .parse()
            .unwrap();

        WorkflowComparison {
            lhs: PartCharacteristic::from_symbol(text.chars().nth(0).unwrap()),
            operator,
            rhs,
        }
    }
}

#[derive(Debug)]
pub struct WorkflowStep {
    comparison: Option<WorkflowComparison>,
    target: WorkflowResult,
}

impl WorkflowStep {
    pub fn from_text(text: &str) -> WorkflowStep {
        let components: Vec<&str> = text.split(':').collect();

        let target = WorkflowResult::from_text(components.last().unwrap());

        match components.len() == 1 {
            true => WorkflowStep {
                comparison: None,
                target,
            },
            _ => {
                let comparison_text = components.first().unwrap();

                WorkflowStep {
                    comparison: Some(WorkflowComparison::from_text(comparison_text)),
                    target,
                }
            }
        }
    }
}

impl WorkflowStep {
    fn apply_to_part(&self, part: &Part) -> Option<WorkflowResult> {
        if self.comparison.is_none() {
            return Some(self.target.clone());
        }

        let WorkflowComparison { lhs, operator, rhs } = &self.comparison.clone().unwrap();

        let characteristic = part.characteristics.get(&lhs).unwrap();

        let result = match operator {
            ComparisonOperator::GreaterThan => characteristic > &rhs,
            _ => characteristic < &rhs,
        };

        match result {
            true => Some(self.target.clone()),
            _ => None,
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
    fn constructs_workflow_step_from_text() {
        let step = WorkflowStep::from_text("a<2168:czk");

        assert!(step.comparison.is_some());

        let comparison = step.comparison.unwrap();

        assert_eq!(comparison.operator, ComparisonOperator::LessThan);
        assert_eq!(comparison.lhs, PartCharacteristic::Aerodynamic);
        assert_eq!(comparison.rhs, 2168);

        assert_eq!(step.target, WorkflowResult::Next(String::from("czk")));
    }
}
