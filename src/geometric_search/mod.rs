use super::symbol_tables::{ST, OrderedST};
use super::symbol_tables::binary_search_tree::{BST, Node};

pub trait RangeSearch1D<K, V>: OrderedST<K, V> {
    /// Insert key-value pair
    fn insert(&mut self, key: K, val: V);
    /// Range search: find all keys between k1 and k2
    fn search(&self, key1: &K, key2: &K) -> Option<Vec<&K>>;
    /// Range count: number of keys between k1 and k2
    fn count(&self, key1: &K, key2: &K) -> usize;
}



pub trait IntervalST<K: Ord, V> {
    type Iter: Iterator;
    /// create interval search tree
    fn new() -> Self;
    /// put interval-value pair into ST
    fn put(&mut self, lo: K, hi: K, val: V);
    /// value paired with given interval
    fn get(&self, lo: &K, hi: &K) -> Option<&V>;
    /// delete the given interval
    fn delete(&mut self, lo: &K, hi: &K);
    /// all intervals that intersect the given interval
    fn intersects(&self, lo: &K, hi: &K) -> Self::Iter;
}


impl<K: Ord, V> RangeSearch1D<K, V> for BST<K, V> {

    #[inline]
    fn insert(&mut self, key: K, val: V) {
        self.put(key, val);
    }

    fn search(&self, lo: &K, hi: &K) -> Option<Vec<&K>> {
        let mut result = Vec::new();


    }


    fn count(&self, lo: &K, hi: &K) -> usize {
        if self.contains(hi) {
            self.rank(hi) - self.rank(lo) + 1
        } else {
            // number of keys < hi
            self.rank(hi) - self.rank(lo)
        }
    }
}


#[test]
fn test_range_search_1d() {
    let mut ost = BST::<char,()>::new();
    ost.insert('B', ());
    ost.insert('D', ());
    ost.insert('A', ());
    ost.insert('I', ());
    ost.insert('H', ());
    ost.insert('F', ());
    ost.insert('P', ());

    assert_eq!(ost.count(&'G', &'K'), 2);
}
