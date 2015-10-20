use std::hash::Hash;
use super::searching::hash_tables::separate_chaining::SeparateChainingHashST;

/// Mathematical set. A collection of distinct keys
pub trait Set<K> {
    /// create an empty set
    fn new() -> Self;
    /// add the key to the set
    fn add(&mut self, key: K);
    /// remove the key from the set
    fn remove(&mut self, key: &K);
    /// return the number of keys in the set
    fn size(&self) -> usize;
    // iterator through keys in the set
    // fn iter(&self) -> Interator<&K>
}


pub type HashSet<K> = SeparateChainingHashST<K,()>;

impl<K: Hash + PartialEq> Set<K> for HashSet<K> {
    fn new() -> Self {
        Self::new()
    }

    fn add(&mut self, key: K) {
        self.put(key, ());
    }

    fn remove(&mut self, key: &K) {
        self.delete(key);
    }

    fn size(&self) -> usize {
        self.size()
    }
}


#[test]
fn test_hash_set() {
    let mut s = HashSet::new();
    assert_eq!(s.size(), 0);
    s.add(100);
    s.add(100);
    s.add(200);
    assert_eq!(s.size(), 2);
    s.remove(&100);
    assert_eq!(s.size(), 1);
}
