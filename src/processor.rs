use std::sync::mpsc::{SyncSender,Sender,Receiver};
use std::sync::mpsc::channel;
use super::GlobalState;

pub trait Process
{
    fn iterate(&mut self, &mut Receiver<GlobalState>, &mut SyncSender<GlobalState> );
}


pub struct Processor<T: Process>
{
    process:T,
    individual_receiver: Receiver<GlobalState>,
    individual_sender: SyncSender<GlobalState>
}

impl<T:Process> Processor<T>{
    pub fn init(&mut self) -> (Sender<Receiver<GlobalState>>)
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
