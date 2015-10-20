// use std::iter::Iterator;

// Symbol Tables
pub trait ST<K, V> {
    /// create a symbol table
    fn new() -> Self;
    /// put key-value pair into the table
    /// (remove key from table if value is null)
    /// a[key] = val;
    fn put(&mut self, key: K, val: V);
    /// value paired with key
    /// (null if key is absent)
    /// a[key]
    fn get(&self, key: &K) -> Option<&V>;
    // FIXME: helper for a[key] =  val
    // fn get_mut(&mut self, key: &K) -> Option<&mut V>;
    /// remove key (and its value) from table
    fn delete(&mut self, key: &K);
    /// is there a value paired with key?
    fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }
    /// is the table empty?
    fn is_empty(&self) -> bool;
    /// number of key-value pairs in the table
    fn size(&self) -> usize;
    // all the keys in the table
    // fn keys() -> Iterator<Item=K>;
}

pub trait OrderedST<K, V>: ST<K, V> {
    /// smallest key
    fn min(&self) -> Option<&K>;

    /// largest key
    fn max(&self) -> Option<&K>;

    /// largest key less than or equal to key
    fn floor(&self, key: &K) -> Option<&K>;

    /// smallest key greater than or equal to key
    fn ceiling(&self, key: &K) -> Option<&K>;

    /// number of keys less than key
    fn rank(&self, key: &K) -> usize;

    /// key of rank k
    fn select(&self, k: usize) -> Option<&K>;

    /// delete smallest key
    fn delete_min(&mut self);

    /// delete largest key
    fn delete_max(&mut self);

    /// number of keys in [lo..hi]
    fn size_of_key_range(&self, lo: &K, hi: &K) -> usize {
        self.rank(hi) - self.rank(lo) + 1
    }
}

pub mod linked_st;

pub mod ordered_array_st;

pub mod binary_search_tree;

pub mod red_black_tree;

pub mod hash_tables;
