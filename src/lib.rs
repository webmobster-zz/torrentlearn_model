#![deny(warnings)]
extern crate rand;
pub mod processor;
pub mod operator;

mod state;
mod graph;

pub use state::GlobalState;
pub use state::LocalState;
pub use operator::UUID;
pub use operator::OperatorMap;
pub use operator::RandomKey;
pub use graph::Graph;
pub use graph::Node;
