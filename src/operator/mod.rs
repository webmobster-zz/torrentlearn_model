use super::FitnessEvaluator;
use std::str::FromStr;
use std::fmt;
use rand::Rng;
use std::sync::Arc;

/// Used to contain the information shared with the operator provided, wrapped in an RC
/// to keep track of uses and when the operator provider can drop whatever is holding the
/// code the function pointer references
pub trait DropHelper{}

pub trait OperatorProvider
{
	fn get_random(&self) -> Operator;
	fn get_slice(&self) -> &[Operator];
	//dynamic dispatch as no paramitzed types in a trait
	fn random(&self,rng: &mut Rng) ->Operator;
	fn random_with_successors(&self,rng: &mut Rng, suc: u8) -> Operator;
}

pub struct Operator
{
	uuid: UUID,
	special: SpecialOperator,
	successors: u8,
	op: fn(&mut [u8]) -> bool,
    drop_helper: Arc<Box<DropHelper>>
}
impl Operator
{
	pub fn get_sucessors(&self) -> u8
	{
		self.successors
	}

}

impl Clone for Operator
{
	fn clone(&self) -> Operator {
        Operator{ uuid: self.uuid, special: self.special, op: self.op, successors: self.successors, drop_helper: self.drop_helper.clone()}
	}
}

impl fmt::Debug for Operator
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Operator")
			.field("uuid", &self.uuid)
	        .field("special", &self.special)
			.field("sucessors", &self.successors)
	        .finish()
		}
}

//TODO UUID string representation should be a fixed size hexadecimal string for both debug and
//display
#[derive(Clone,Debug,Copy,Eq,PartialEq,Hash)]
pub struct UUID {pub x: [u64;2]}
impl FromStr for UUID {
	type Err = ();

            #[inline]
            #[allow(deprecated)]
            fn from_str(src: &str) -> Result<Self, ()> {
		let mut split = src.split(",");
                let part1 = split.next().unwrap();
		let part2 = split.next().unwrap();
		Ok(UUID{x:[part1.parse::<u64>().unwrap(),part2.parse::<u64>().unwrap()]})

            }
}

impl fmt::Display for UUID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}_{}", self.x[0], self.x[1])
    }
}



#[derive(Clone,Debug,Copy)]
pub enum SpecialOperator
{
	None,
	NewThread,
	Send,
	Receive,
	CopyToGlobal,
	CopyFromGlobal,

}
