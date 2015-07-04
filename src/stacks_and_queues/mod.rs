// stacks
pub mod linked_stack;
pub mod fixed_capacity_stack;
pub mod resizing_array_stack;

pub trait StackOfStrings {
    fn new() -> Self;
    fn push(&mut self, item: String);
    fn pop(&mut self) ->  String;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize {
        unimplemented!()
    }
}

// queues
pub mod linked_queue;
pub mod resizing_array_queue;


pub trait QueueOfStrings {
    /// create an empty queue
    fn new() -> Self;
    /// insert a new string onto queue
    fn enqueue(&mut self, item: String);
    /// remove and return the string least recently added
    fn dequeue(&mut self) -> String;
    /// is the queue empty?
    fn is_empty(&self) -> bool;
    /// number of strings on the queue
    fn size(&self) -> usize {
        unimplemented!()
    }
}
