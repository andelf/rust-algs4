pub mod quick_find;
pub mod quick_union;
pub mod weighted_quick_union;
pub mod improved;

pub trait UnionFind {
    fn new(n: usize) -> Self;

    // whose id equals id[p] to id[q].
    fn union(&mut self, p: usize, q: usize);

    // Check if p and q have the same id.
    fn connected(&self, p: usize, q: usize) -> bool;

    #[allow(unused_variables)]
    fn find(&self, p: usize) -> usize {
        unimplemented!()
    }

    fn count(&self) -> usize {
        unimplemented!()
    }
}
