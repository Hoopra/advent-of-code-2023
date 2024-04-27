mod comparison;
mod step;
mod workflow;

pub use comparison::{ComparisonOperator, WorkflowComparison};
pub use step::WorkflowStep;
pub use workflow::{parse_workflows, WorkflowResult};
