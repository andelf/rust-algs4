// use std::iter::Iterator;
use std::collections::LinkedList;
use super::ST;

pub struct LinkedST<K, V> {
    /// Maintain an (unordered) linked list of key-value pairs.
    t: LinkedList<(K,V)>
}


impl<K: Eq, V> ST<K,V> for LinkedST<K, V> {
    fn new() -> Self {
        LinkedST {
            t: LinkedList::new()
        }
    }

    fn put(&mut self, key: K, val: V) {
        self.t.push_front((key,val))
    }

    fn get(&self, key: &K) -> Option<&V> {
        for &(ref k, ref v) in self.t.iter() {
            if k == key {
                return Some(v)
            }
        }
        None
    }

    fn delete(&mut self, key: &K) {
        let mut i = 0;
        let mut found = false;
        for &(ref k, _) in self.t.iter() {
            if k == key {
                found = true;
                break
            } else {
                i += 1;
            }
        }

        if found {
            let mut remains = self.t.split_off(i);
            // delete i-th
            self.t.pop_back();
            self.t.append(&mut remains)
        }
    }

    /// is the table empty?
    fn is_empty(&self) -> bool {
        self.t.len() == 0
    }

    fn size(&self) -> usize {
        self.t.len()
    }

    // fn keys() -> Iterator<Item=K> {
    //     unimplemented!()
    // }
}


#[test]
fn test_linked_symbol_table() {
    let mut st: LinkedST<char,usize> = ST::new();
    assert!(st.is_empty());
    for (i, c) in "SEARCHEXAMPLE".chars().enumerate() {
        st.put(c, i);
    }

    assert_eq!(st.get(&'X'), Some(&7));
    assert_eq!(st.get(&'E'), Some(&12));
    assert_eq!(st.size(), 13);
    assert_eq!(st.is_empty(), false);
}
