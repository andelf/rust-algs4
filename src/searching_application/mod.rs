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
