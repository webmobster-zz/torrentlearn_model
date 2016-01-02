#![deny(warnings)]
extern crate rand;
pub mod processor;
pub mod operator;

mod state;
mod graph;

pub use state::GlobalMemory;
pub use state::GlobalState;
pub use state::LocalState;
pub use state::FitnessEvaluator;

pub use operator::UUID;
pub use operator::Operator;
pub use operator::SpecialOperator;
pub use operator::OperatorProvider;

pub use graph::Graph;
pub use graph::Node;
