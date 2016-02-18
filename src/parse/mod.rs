pub use self::parsetree::ParseTree;
pub use self::parsetree::Data;
pub use self::parsetree::Statement;
pub use self::parsetree::ConditionalStatement;
pub use self::parsetree::Position;
use rand::Rng;
use Operator;

mod parsetree;

#[derive(Copy,Clone)]
pub enum SingleOperators
{
    Add
}
#[derive(Copy,Clone)]
pub enum ConditionalOperators
{
    Equal
}
#[derive(Copy,Clone)]
pub enum VecOperators
{
    Add
}
#[derive(Copy,Clone)]
pub enum MapOperators
{
    Add
}
#[derive(Copy,Clone)]
pub enum ReduceOperators
{
    Sum
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
pub enum AllOperators
{
    Single(SingleOperators),
    Conditional(ConditionalOperators),
    Vec(VecOperators),
    Map(MapOperators),
    Reduce(ReduceOperators),
    Special(SpecialOperator)
}
pub enum GeneratedResult{
    Tree(ParseTree),
    SpecialOperator(Operator)
}

pub fn generate_function<T: Rng>(enabled_operators: &Vec<(AllOperators,f32)>, cost_calculator: fn(&AllOperators)-> u64, rng: &mut T) -> GeneratedResult {
    let mut generated_operator: Option<AllOperators> =None;
	let sample = rng.gen::<f32>();

	//unchecked for correctness
	let mut running_total = 0.0;

	for i in 0..enabled_operators.len()
	{
		running_total+=enabled_operators[i].1;


		if sample < running_total
		{
			generated_operator=Some(enabled_operators[i].0);
            break
		}
	}
    match generated_operator.unwrap()
    {
        AllOperators::Special(special) => GeneratedResult::SpecialOperator(generate_special_operator(special,cost_calculator)),
        _ => unimplemented!()//parsetree::generate_parse_tree(generated_operator)
    }
}
fn generate_special_operator(special: SpecialOperator, cost_calculator: fn(&AllOperators)-> u64) -> Operator
{
    let sucessors;
    match special{
        SpecialOperator::NewThread => { sucessors=2;}
        _=> unimplemented!()
    }
    Operator{ cost: cost_calculator(&AllOperators::Special(special)), special: special, op: dummy, successors: sucessors, drop_helper: None, parts: None}
}

pub fn dummy(_:&mut [u8]) ->bool
{
    panic!("Should never be called")
}
