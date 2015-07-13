// Week 4: Priority Queues


pub trait MaxPQ<Key: PartialEq> {
    /// create an empty priority queue
    fn new() -> Self;
    /// create a priority queue with given keys
    fn from_vec(a: Vec<Key>) -> Self;
    /// insert a key into the priority queue
    fn insert(&mut self, v: Key);
    /// return and remove the largest key
    fn del_max(&mut self) -> Option<Key>;
    /// is the priority queue empty?
    fn is_empty(&self) -> bool;
    /// return the largest key
    fn max(&self) -> Option<&Key>;
    /// number of entries in the priority queue
    fn size(&self) -> usize;
}


pub trait MinPQ<Key: PartialEq> {
    /// create an empty priority queue
    fn new() -> Self;
    /// create a priority queue with given keys
    fn from_vec(a: Vec<Key>) -> Self;
    /// insert a key into the priority queue
    fn insert(&mut self, v: Key);
    /// return and remove the smallest key
    fn del_min(&mut self) -> Option<Key>;
    /// is the priority queue empty?
    fn is_empty(&self) -> bool;
    /// return the smallest key
    fn min(&self) -> Option<&Key>;
    /// number of entries in the priority queue
    fn size(&self) -> usize;
}


pub mod unordered;

pub mod binary_heaps;

pub mod heapsort;
