pub trait RangeSearch1D<K, V> {
    /// Insert key-value pair
    fn new();
    /// Search for key k
    fn insert(&mut self, key: K, val: V);
    /// Delete key k
    fn delete(&mut self, key: &K);
    /// Range search: find all keys between k1 and k2
    fn search(&self, key1: &K, key2: &K) -> Option<Vec<&K>>;
    /// Range count: number of keys between k1 and k2
    fn count(&self, key1: &K, key2: &K) -> usize;
}
