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
        self.get(key).is_none()
    }
    /// is the table empty?
    fn is_empty(&self) -> bool;
    /// number of key-value pairs in the table
    fn size(&self) -> usize;
    // all the keys in the table
    // fn keys() -> Iterator<Item=K>;
}


pub mod linked_st;

pub mod ordered_array_st;
