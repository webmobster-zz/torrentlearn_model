use super::Graph;
use std::cmp;
use std::cmp::Ordering::{Less,Equal,Greater};
use std::sync::{Arc};
use std::sync::atomic::AtomicUsize;
use std::sync::atomic;

pub const ARRAY_SIZE: usize = 1000;
const DEFAULT_ORDERING: atomic::Ordering = atomic::Ordering::SeqCst;

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
	pub life: Option<Arc<AtomicUsize>>,

	//mutable by multiple threads
	//pub comm: Option<Arc<Mutex<BiChannel<StateIO>>>>,
	pub thread_count: Option<Arc<AtomicUsize>>,


	//Persistant after evaluation but not after selection
	pub fitness: Option<Arc<AtomicUsize>>,

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
		self.life.is_some() && self.thread_count.as_ref().unwrap().load(DEFAULT_ORDERING) == 0 && self.fitness.is_some()
	}

	pub fn cleanup(&mut self)
	{
		self.fitness_evaluator=None;
	}


	pub fn initialize(&mut self,life: u64, problem: T )
	{
		self.life=Some(Arc::new(AtomicUsize::new(life as usize)));
		self.thread_count=None;
		self.fitness=None;
		self.fitness_evaluator=Some(problem);
	}

	//FIXME:  This needs to be reworked
	pub fn unique_graphvec_copy(&self) -> (Graph,Vec<u64>)
	{
		(self.graph.clone(),self.vec.clone())

	}
	pub fn get_fitness(&self) -> u64
	{
		self.fitness.as_ref().unwrap().load(DEFAULT_ORDERING) as u64
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

	fn partial_cmp(&self, other: &GlobalState<T>) -> Option<cmp::Ordering>
	{

		if (self.fitness.as_ref().unwrap().load(DEFAULT_ORDERING) as u64)  < (other.fitness.as_ref().unwrap().load(DEFAULT_ORDERING) as u64) {Some(Less)}
    		else if self.fitness.as_ref().unwrap().load(DEFAULT_ORDERING) as u64  > other.fitness.as_ref().unwrap().load(DEFAULT_ORDERING) as u64 { Some(Greater) }
    		else { Some(Equal) }

	}


}



impl<T: FitnessEvaluator> PartialEq for GlobalState<T>
{

	fn  eq(&self, other: &GlobalState<T>) -> bool
	{

		if (self.fitness.as_ref().unwrap().load(DEFAULT_ORDERING) as u64)  == (other.fitness.as_ref().unwrap().load(DEFAULT_ORDERING) as u64) { true }
    		else {false}

	}


}

impl<T: FitnessEvaluator> Ord for GlobalState<T>
{

	fn cmp(&self, other: &GlobalState<T>) -> cmp::Ordering
	{

        self.partial_cmp(other).unwrap()


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
