use super::Graph;

use std::cmp::Ordering;
use std::cmp::Ordering::{Less,Equal,Greater};
use std::sync::{Arc, Mutex};


const ARRAY_SIZE: usize = 1000;


pub trait FitnessEvaluator
{
	fn intialize(&mut self);
	fn send_byte(&mut self, byte: u8);
	fn get_byte(&mut self, byte: u8);
	fn finish(&mut self) -> u64;

}

pub type GlobalMemory = Vec<u64>;

//FIXME: Add back in a way of evaluating, probably not a billion comm channels
pub struct GlobalState<T: FitnessEvaluator>
{
	//Persistant after evaluation and after selection and mutable by multiple threads
	//using unsafe methods
	//TODO: Think about this
	pub vec: GlobalMemory,

	pub graph: Graph,
	pub life: Option<Arc<Mutex<u64>>>,

	//mutable by multiple threads
	//pub comm: Option<Arc<Mutex<BiChannel<StateIO>>>>,
	pub thread_count: Option<Arc<Mutex<u64>>>,


	//Persistant after evaluation but not after selection
	pub fitness: Option<Arc<Mutex<u64>>>,

	pub fitness_evaluator: Option<T>

}

impl<T: FitnessEvaluator> Clone for GlobalState<T>
{
	fn clone(&self) ->GlobalState<T>
	{
		GlobalState{vec: self.vec.clone(), life: self.life.clone(),graph: self.graph.clone(),thread_count: self.thread_count.clone(),fitness: self.fitness.clone(),fitness_evaluator: None}
	}

}

impl<T: FitnessEvaluator> GlobalState<T>
{


	pub fn new(memory: Vec<u64>, graph: Graph) -> GlobalState<T>
	{

		GlobalState{vec: memory, life: None,graph: graph, fitness: None, thread_count: None, fitness_evaluator: None}

	}

	pub fn evaluated_is_sane(&self ) -> bool
	{
		self.life.is_some() && *self.thread_count.as_ref().unwrap().lock().unwrap() == 0 && self.fitness.is_some()
	}

	pub fn cleanup(&mut self)
	{
		self.fitness_evaluator=None;
	}


	pub fn initialize(&mut self,life: u64, problem: T )
	{
		self.life=Some(Arc::new(Mutex::new(life)));
		self.thread_count=None;
		self.fitness=None;
		self.fitness_evaluator=Some(problem);
	}

	//FIXME
	pub fn unique_graphvec_copy(&self) -> (Graph,Vec<u64>)
	{
		(self.graph.clone(),self.vec.clone())

	}
	pub fn get_fitness(&self) -> u64
	{
		let fitness = self.fitness.clone().unwrap();
		let lockfit =  *fitness.lock().unwrap();
		lockfit
	}

}


//These are all to allow sorting based on fitness
//FIXME: This is probably a bad idea
//Need to remeber to not deadlock if self and other are the same
impl<T: FitnessEvaluator> Eq for GlobalState<T>
{

}

impl<T: FitnessEvaluator> PartialOrd for GlobalState<T>
{

	fn partial_cmp(&self, other: &GlobalState<T>) -> Option<Ordering>
	{
		let lockself: u64;
		let lockother: u64;
		{
			let fitness_self = self.fitness.clone().unwrap();
			lockself =  *fitness_self.lock().unwrap();

		}
		{
			let fitness_other = other.fitness.clone().unwrap();
			lockother =  *fitness_other.lock().unwrap();
		}

		if lockself < lockother { Some(Less) }
    		else if lockself > lockother { Some(Greater) }
    		else { Some(Equal) }

	}


}



impl<T: FitnessEvaluator> PartialEq for GlobalState<T>
{

	fn  eq(&self, other: &GlobalState<T>) -> bool
	{
		let lockself: u64;
		let lockother: u64;
		{
			let fitness_self = self.fitness.clone().unwrap();
			lockself =  *fitness_self.lock().unwrap();

		}
		{
			let fitness_other = other.fitness.clone().unwrap();
			lockother =  *fitness_other.lock().unwrap();
		}

		if lockself == lockother { true }
    		else {false}

	}


}

impl<T: FitnessEvaluator> Ord for GlobalState<T>
{

	fn cmp(&self, other: &GlobalState<T>) -> Ordering
	{


		let lockself: u64;
		let lockother: u64;
		{
			let fitness_self = self.fitness.clone().unwrap();
			lockself =  *fitness_self.lock().unwrap();

		}
		{
			let fitness_other = other.fitness.clone().unwrap();
			lockother =  *fitness_other.lock().unwrap();
		}

		if lockself < lockother { Less }
    		else if lockself > lockother { Greater }
    		else { Equal }

	}


}

#[derive(Copy)]
pub struct LocalState
{
	pub node: Option<usize>,
	pub local_array: [u8;ARRAY_SIZE]

}

impl LocalState
{
	pub fn new() -> LocalState
	{

		LocalState{node: Some(0), local_array: [0; ARRAY_SIZE]}

	}

}

impl Clone for LocalState
{
	fn clone(&self) -> LocalState
	{
		LocalState{node: self.node, local_array: self.local_array}
	}

}
