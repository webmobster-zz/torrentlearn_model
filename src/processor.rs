use std::sync::mpsc::{SyncSender,Sender,Receiver};
use std::sync::mpsc::channel;
use super::GlobalState;
use super::FitnessEvaluator;



pub trait Process<T: FitnessEvaluator>
{
    fn iterate(&mut self, &mut Receiver<GlobalState<T>>, &mut SyncSender<GlobalState<T>> );
}


pub struct Processor<U: FitnessEvaluator, T: Process<U>>
{
    process:T,
    individual_receiver: Receiver<GlobalState<U>>,
    individual_sender: SyncSender<GlobalState<U>>
}

impl<U: FitnessEvaluator, T:Process<U>> Processor<U,T>{
    pub fn init(&mut self) -> (Sender<Receiver<GlobalState<U>>>)
    {
        let (send, _) = channel();
        send
    }

    pub fn run(&mut self)
    {
        loop {
            self.process.iterate(&mut self.individual_receiver, &mut self.individual_sender);
        }
    }
}
