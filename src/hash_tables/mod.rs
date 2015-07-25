use std::hash::{Hash, Hasher, SipHasher};
use std::borrow::Borrow;


pub mod linear_probing;

struct Node<K, V> {
    key: K,
    val: V,
    next: Option<Box<Node<K, V>>>
}

pub struct SeparateChainingHashST<K, V> {
    st: Vec<Option<Box<Node<K, V>>>>
}


const M: usize = 97;

impl<K: Hash + PartialEq, V> SeparateChainingHashST<K, V> {
    pub fn new() -> SeparateChainingHashST<K, V> {
        let mut st = Vec::with_capacity(M);
        for _ in 0 .. M {
            st.push(None);
        }
        SeparateChainingHashST {
            st: st
        }
    }

    fn hash(key: &K) -> usize {
        let mut hasher = SipHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % M
    }

    pub fn get<T: Borrow<K>>(&self, key: T) -> Option<&V> {
        let key = key.borrow();
        let i = Self::hash(key);
        let mut x = self.st[i].as_ref();
        while x.is_some() {
            if *key == x.unwrap().key {
                return Some(&x.unwrap().val)
            }
            x = x.unwrap().next.as_ref();
        }
        None
    }

    pub fn put(&mut self, key: K, val: V) {
        let i = Self::hash(&key);
        {
            let mut x = self.st[i].as_mut();
            while x.is_some() {
                if key.eq(&x.as_ref().unwrap().key) {
                    x.unwrap().val = val;
                    return;
                } else {
                    x = x.unwrap().next.as_mut();
                }
            }
        }
        let old = self.st[i].take();
        self.st[i] = Some(Box::new(Node { key: key, val: val, next: old }))
    }
}


#[test]
fn test_separate_chaining_st() {
    let mut m = SeparateChainingHashST::new();
    m.put("Name", "Feather");
    m.put("Age", "25");
    m.put("Address", "Beijing");

    assert_eq!(m.get("Age"), Some(&"25"));
    assert_eq!(m.get("Gender"), None);
}
