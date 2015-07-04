use std::iter::Iterator;

// stacks
pub mod linked_stack;
pub mod fixed_capacity_stack;
pub mod resizing_array_stack;

pub trait StackOfStrings {
    /// create an empty stack
    fn new() -> Self;
    /// insert a new string onto stack
    fn push(&mut self, item: String);
    /// remove and return the string most recently added
    fn pop(&mut self) ->  String;
    /// is the stack empty?
    fn is_empty(&self) -> bool;
    /// number of strings on the stack
    fn size(&self) -> usize {
        unimplemented!()
    }
}

/// generic stack
pub trait Stack<T> {
    /// create an empty stack
    fn new() -> Self;
    /// insert a new item onto stack
    fn push(&mut self, item: T);
    /// remove and return the item most recently added
    fn pop(&mut self) ->  T;
    /// is the stack empty?
    fn is_empty(&self) -> bool;
    /// number of strings on the stack
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

/// generic queue
pub trait Queue<T> {
    /// create an empty queue
    fn new() -> Self;
    /// insert a new item onto queue
    fn enqueue(&mut self, item: T);
    /// remove and return the item least recently added
    fn dequeue(&mut self) -> T;
    /// is the queue empty?
    fn is_empty(&self) -> bool;
    /// number of items on the queue
    fn size(&self) -> usize {
        unimplemented!()
    }
}

// bag
pub trait Bag<T> {
    fn new() -> Self;
    fn add(&mut self, item: T);
    fn iter(&self) -> Iterator<Item=T>;
    fn size(&self) -> usize {
        unimplemented!()
    }
}
