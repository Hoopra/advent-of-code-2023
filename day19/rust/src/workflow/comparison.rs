use crate::{
    constants::{MAX_RATING, MIN_RATING},
    part::PartFeature,
};

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
    pub lhs: PartFeature,
    pub operator: ComparisonOperator,
    pub rhs: u32,
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
            lhs: PartFeature::from_symbol(text.chars().nth(0).unwrap()),
            operator,
            rhs,
        }
    }
}

impl WorkflowComparison {
    pub fn bounds(&self) -> (u64, u64) {
        let value = self.rhs as u64;

        let (lower, upper) = match self.operator {
            ComparisonOperator::LessThan => (MIN_RATING, value - 1),
            ComparisonOperator::GreaterThan => (value + 1, MAX_RATING),
        };

        (lower, upper)
    }

    pub fn reverse_bounds(&self) -> (u64, u64) {
        let value = self.rhs as u64;

        let (lower, upper) = match self.operator {
            ComparisonOperator::GreaterThan => (MIN_RATING, value),
            ComparisonOperator::LessThan => (value, MAX_RATING),
        };

        (lower, upper)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_infers_less_than_bounds() {
        let comparison = WorkflowComparison {
            lhs: PartFeature::Shiny,
            operator: ComparisonOperator::LessThan,
            rhs: 1000,
        };

        let (lower, upper) = comparison.bounds();

        assert_eq!(lower, 1);
        assert_eq!(upper, 999);
    }

    #[test]
    fn correctly_infers_greater_than_bounds() {
        let comparison = WorkflowComparison {
            lhs: PartFeature::Shiny,
            operator: ComparisonOperator::GreaterThan,
            rhs: 3000,
        };

        let (lower, upper) = comparison.bounds();

        assert_eq!(lower, 3001);
        assert_eq!(upper, 4000);
    }
}
