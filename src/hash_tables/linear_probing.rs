use std::hash::{Hash, Hasher, SipHasher};
use std::borrow::Borrow;

const M: usize = 30001;

pub struct LinearProbingHashST<K, V> {
    keys: Vec<Option<K>>,
    vals: Vec<Option<V>>
}

impl<K: Hash + PartialEq, V> LinearProbingHashST<K, V> {
    pub fn new() -> LinearProbingHashST<K, V> {
        let mut vals = Vec::with_capacity(M);
        let mut keys = Vec::with_capacity(M);
        // FIXME: How to initialize
        for _ in 0 .. M {
            keys.push(None);
            vals.push(None);
        }
        LinearProbingHashST {
            vals: vals,
            keys: keys
        }
    }

    fn hash(key: &K) -> usize {
        let mut hasher = SipHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % M
    }

    pub fn put(&mut self, key: K, val: V) {
        let mut i = Self::hash(&key);
        while self.keys[i].is_some() {
            if self.keys[i].as_ref().unwrap().eq(&key) {
                break;
            }
            i = (i+1) % M;
        }
        self.keys[i] = Some(key);
        self.vals[i] = Some(val);
    }

    pub fn get<T: Borrow<K>>(&self, key: T) -> Option<&V> {
        let key = key.borrow();
        let mut i = Self::hash(key);
        while self.keys[i].is_some() {
            if self.keys[i].as_ref().unwrap().eq(key) {
                return self.vals[i].as_ref();
            }
            i = (i+1) % M;
        }
        None
    }
}


#[test]
fn test_linear_probing_st() {
    let mut m = LinearProbingHashST::new();
    m.put("Name", "Feather");
    m.put("Age", "25");
    m.put("Address", "Beijing");

    assert_eq!(m.get("Age"), Some(&"25"));
    assert_eq!(m.get("Gender"), None);
}
