use super::symbol_tables::{ST, OrderedST};
use super::symbol_tables::binary_search_tree::{BST, Node};

pub mod primitive;

pub mod kd_tree;

pub mod interval_search_tree;

pub trait RangeSearch1D<K, V>: OrderedST<K, V> {
    /// Insert key-value pair
    fn insert(&mut self, key: K, val: V);
    /// Range search: find all keys between k1 and k2
    fn range_search(&self, key1: &K, key2: &K) -> Option<Vec<&K>>;
    /// Range count: number of keys between k1 and k2
    fn range_count(&self, key1: &K, key2: &K) -> usize;
}

pub trait OrthogonalRangeSearch2D<K, V>: OrderedST<(K,K), V> {
    /// Insert a 2d key
    fn insert(&mut self, key: (K,K), val: V);
    /// Range search: find all keys that lie in a 2d range
    fn range_search(&self, left_top: (K,K), right_bottom: (K,K)) -> Option<Vec<&(K,K)>>;
    /// Range count: number of keys that lie in a 2d range
    fn range_count(&self, left_top: (K,K), right_bottom: (K,K)) -> usize;
}

// pub trait IntervalST<K, V> {
//     type Iter: Iterator;
//     /// create interval range_search tree
//     fn new() -> Self;
//     /// put interval-value pair into ST
//     fn put(&mut self, lo: K, hi: K, val: V);
//     /// value paired with given interval
//     fn get(&self, lo: &K, hi: &K) -> Option<&V>;
//     /// delete the given interval
//     fn delete(&mut self, lo: &K, hi: &K);
//     /// all intervals that intersect the given interval
//     fn intersects(&self, lo: &K, hi: &K) -> Self::Iter;
// }


impl<K: PartialOrd, V> RangeSearch1D<K, V> for BST<K, V> {

    #[inline]
    fn insert(&mut self, key: K, val: V) {
        self.put(key, val);
    }

    fn range_search(&self, lo: &K, hi: &K) -> Option<Vec<&K>> {
        let mut queue: Vec<&K> = Vec::new();
        fn inorder<'a, K: PartialOrd, V>(x: Option<&'a Box<Node<K,V>>>, queue: &mut Vec<&'a K>, lo: &K, hi: &K) {
            if x.is_none() {
                return;
            }
            x.map(|n| inorder(n.left.as_ref(), queue, lo, hi));
            if x.map(|n| n.key >= *lo && n.key <= *hi ).unwrap_or(false) {
               x.map(|n| queue.push(&n.key));
            }
            x.map(|n| inorder(n.right.as_ref(), queue, lo, hi));
        };
        inorder(self.root.as_ref(), &mut queue, lo, hi);
        if queue.is_empty() {
            None
        } else {
            Some(queue)
        }
    }


    fn range_count(&self, lo: &K, hi: &K) -> usize {
        if self.contains(hi) {
            self.rank(hi) - self.rank(lo) + 1
        } else {
            // number of keys < hi
            self.rank(hi) - self.rank(lo)
        }
    }
}


#[test]
fn test_range_range_search_1d() {
    let mut ost = BST::<char,()>::new();
    ost.insert('B', ());
    ost.insert('D', ());
    ost.insert('A', ());
    ost.insert('I', ());
    ost.insert('H', ());
    ost.insert('F', ());
    ost.insert('P', ());

    assert_eq!(ost.range_count(&'G', &'K'), 2);

    assert_eq!(ost.range_search(&'G', &'K'), Some(vec![&'H', &'I']));
    assert_eq!(ost.range_search(&'Y', &'Z'), None);
}
