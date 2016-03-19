pub use self::parsetree::ParseTree;
pub use self::parsetree::Data;
pub use self::parsetree::Statement;
pub use self::parsetree::ConditionalStatement;
pub use self::parsetree::Position;
use rand::distributions::{Weighted, WeightedChoice, IndependentSample};
use rand::Rng;
use Operator;

mod parsetree;

#[derive(Copy,Clone)]
pub enum SingleOperators {
    Add,
}
#[derive(Copy,Clone)]
pub enum ConditionalOperators {
    Equal,
}
#[derive(Copy,Clone)]
pub enum VecOperators {
    Add,
}
#[derive(Copy,Clone)]
pub enum MapOperators {
    Add,
}
#[derive(Copy,Clone)]
pub enum ReduceOperators {
    Sum,
}
#[derive(Clone,Debug,Copy)]
pub enum SpecialOperator {
    None,
    NewThread,
    Send,
    Receive,
    CopyToGlobal,
    CopyFromGlobal,
}
#[derive(Copy,Clone)]
pub enum AllOperators {
    Single(SingleOperators),
    Conditional(ConditionalOperators),
    Special(SpecialOperator),
}
pub enum GeneratedResult {
    Tree(ParseTree),
    SpecialOperator(Operator),
}

pub fn generate_function_with_sucessors<T: Rng>(enabled_operators: &mut Vec<Weighted<AllOperators>>,
                                                cost_calculator: fn(&AllOperators) -> u64,
                                                mut rng: &mut T,
                                                suc: u8)
                                                -> GeneratedResult {
    let wc = WeightedChoice::new(enabled_operators);
    let mut generated_operator: AllOperators;
    //FIXME: Inefficient as fuck
    loop {
        generated_operator = wc.ind_sample(rng);
        if check_successors(generated_operator,suc) {
            break;
        }
    }
    match generated_operator {
        AllOperators::Special(special) => {
            GeneratedResult::SpecialOperator(generate_special_operator(special, cost_calculator))
        }
        //FIXME
        _ => GeneratedResult::Tree(parsetree::ParseTree::generate_parse_tree(generated_operator,rng))
    }
}
pub fn generate_function<T: Rng>(enabled_operators: &mut Vec<Weighted<AllOperators>>,
                                 cost_calculator: fn(&AllOperators) -> u64,
                                 rng: &mut T)
                                 -> GeneratedResult {
    let wc = WeightedChoice::new(enabled_operators);
    let generated_operator: AllOperators = wc.ind_sample(rng);
    match generated_operator {
        AllOperators::Special(special) => {
            GeneratedResult::SpecialOperator(generate_special_operator(special, cost_calculator))
        }
        //FIXME
        _ => GeneratedResult::Tree(parsetree::ParseTree::generate_parse_tree(generated_operator,rng))
    }
}

fn generate_special_operator(special: SpecialOperator,
                             cost_calculator: fn(&AllOperators) -> u64)
                             -> Operator {
    let sucessors;
    match special {
        SpecialOperator::NewThread => {
            sucessors = 2;
        }
        //FIXME
        _ => {sucessors =1},
    }
    Operator {
        cost: cost_calculator(&AllOperators::Special(special)),
        special: special,
        op: dummy,
        successors: sucessors,
        drop_helper: None,
        parts: None,
    }
}

fn check_successors(operator: AllOperators, suc: u8) -> bool{
    let result = match operator {
        AllOperators::Conditional(_) => 2,
        AllOperators::Special(special) => match special {
            SpecialOperator::NewThread => 2,
            //FIXME
            _ => 1
        },
        //FIXME
        _ => 1
    };
    if result == suc {
        true
    } else {
        false
    }
}

pub fn dummy(_: &mut [u8]) -> bool {
    panic!("Should never be called")
}
