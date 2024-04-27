use super::{ComparisonOperator, WorkflowComparison, WorkflowResult};
use crate::part::{Part, PartCombination};

#[derive(Debug)]
pub struct WorkflowStep {
    pub comparison: Option<WorkflowComparison>,
    pub target: WorkflowResult,
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
    pub fn apply_to_part(&self, part: &Part) -> Option<WorkflowResult> {
        if self.comparison.is_none() {
            return Some(self.target.clone());
        }

        let WorkflowComparison { lhs, operator, rhs } = &self.comparison.clone().unwrap();

        let characteristic = part.features.get(&lhs).unwrap();

        let result = match operator {
            ComparisonOperator::GreaterThan => characteristic > &rhs,
            _ => characteristic < &rhs,
        };

        match result {
            true => Some(self.target.clone()),
            _ => None,
        }
    }

    pub fn narrow_by_comparison(&self, part: &PartCombination) -> PartCombination {
        let destination = self.target.clone();

        match &self.comparison {
            None => PartCombination::with_destination(part, destination),
            Some(comparison) => {
                let (lower, upper) = comparison.bounds();

                PartCombination::with_narrowed_combinations(
                    part,
                    comparison.lhs.clone(),
                    lower,
                    upper,
                    destination,
                )
            }
        }
    }

    pub fn narrow_by_reverse_comparison(&self, part: &PartCombination) -> PartCombination {
        let destination = self.target.clone();

        match &self.comparison {
            None => PartCombination::with_destination(part, destination),
            Some(comparison) => {
                let (lower, upper) = comparison.reverse_bounds();

                PartCombination::with_narrowed_combinations(
                    part,
                    comparison.lhs.clone(),
                    lower,
                    upper,
                    destination,
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::part::PartFeature;

    #[test]
    fn constructs_workflow_step_from_text() {
        let step = WorkflowStep::from_text("a<2168:czk");

        assert!(step.comparison.is_some());

        let comparison = step.comparison.unwrap();

        assert_eq!(comparison.operator, ComparisonOperator::LessThan);
        assert_eq!(comparison.lhs, PartFeature::Aerodynamic);
        assert_eq!(comparison.rhs, 2168);

        assert_eq!(step.target, WorkflowResult::Next(String::from("czk")));
    }
}
