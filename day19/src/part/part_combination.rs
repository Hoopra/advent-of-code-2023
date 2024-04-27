use crate::{
    constants::{MAX_RATING, MIN_RATING},
    workflow::WorkflowResult,
};

use super::PartFeature;
use std::collections::HashMap;

type Range = (u64, u64);

#[derive(Debug, PartialEq, Clone)]
pub struct PartCombination {
    pub features: HashMap<PartFeature, Range>,
    pub destination: WorkflowResult,
}

impl PartCombination {
    pub fn new() -> PartCombination {
        let mut features = HashMap::new();

        features.insert(PartFeature::Aerodynamic, (MIN_RATING, MAX_RATING));
        features.insert(PartFeature::ExtremelyCool, (MIN_RATING, MAX_RATING));
        features.insert(PartFeature::Musical, (MIN_RATING, MAX_RATING));
        features.insert(PartFeature::Shiny, (MIN_RATING, MAX_RATING));

        PartCombination {
            features,
            destination: WorkflowResult::Next(String::from("in")),
        }
    }

    #[cfg(test)]
    pub fn with_values(
        aerodynamic: Range,
        coolness: Range,
        musical: Range,
        shiny: Range,
        destination: WorkflowResult,
    ) -> PartCombination {
        let mut features = HashMap::new();

        features.insert(PartFeature::Aerodynamic, aerodynamic);
        features.insert(PartFeature::ExtremelyCool, coolness);
        features.insert(PartFeature::Musical, musical);
        features.insert(PartFeature::Shiny, shiny);

        PartCombination {
            features,
            destination,
        }
    }

    pub fn with_narrowed_combinations(
        part: &PartCombination,
        feature: PartFeature,
        lower: u64,
        upper: u64,
        destination: WorkflowResult,
    ) -> PartCombination {
        let mut features = part.features.clone();

        let current = features.get(&feature).unwrap_or(&(MIN_RATING, MAX_RATING));

        let (current_lower, current_upper) = current;

        let lower = u64::max(lower, *current_lower);
        let upper = u64::min(upper, *current_upper);

        features.insert(feature, (lower, upper));

        PartCombination {
            features,
            destination,
        }
    }
    pub fn with_destination(
        part: &PartCombination,
        destination: WorkflowResult,
    ) -> PartCombination {
        PartCombination {
            features: part.features.clone(),
            destination,
        }
    }
}

impl PartCombination {
    pub fn get_combinations(&self) -> u64 {
        self.features
            .values()
            .fold(1, |product, (lower, upper)| match upper >= lower {
                true => product * (upper - lower + 1),
                _ => 0,
            })
    }

    pub fn is_valid(&self) -> bool {
        self.features.values().all(|(lower, upper)| upper >= lower)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_combinations() {
        let part = PartCombination::new();

        assert_eq!(part.get_combinations(), 4000 * 4000 * 4000 * 4000);

        let part = PartCombination::with_values(
            (1, 500),
            (1, 4000),
            (2000, 2100),
            (3000, 4000),
            WorkflowResult::Accept,
        );

        assert_eq!(part.get_combinations(), 500 * 4000 * 101 * 1001);
    }
}
