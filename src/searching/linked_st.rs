// use std::iter::Iterator;
use std::ops::Index;
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
        for &mut (ref mut k, ref mut v) in self.t.iter_mut() {
            if k == &key {
                *v = val;
                return;
            }
        }
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

    // FIXME: can't be used to add new pairs
    // fn get_mut(&mut self, key: &K) -> Option<&mut V> {
    //     for pairs in self.t.iter_mut() {
    //         if &pairs.0 == key {
    //             return Some(&mut pairs.1)
    //         }
    //     }
    //     None
    // }

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


impl<K: Eq, V> Index<K> for LinkedST<K, V> {
    type Output = V;

    fn index<'a>(&'a self, index: K) -> &'a Self::Output {
        self.get(&index).unwrap()
    }
}

// impl<K: Eq, V> IndexMut<K> for LinkedST<K, V> {
//     fn index_mut<'a>(&'a mut self, index: K) -> &'a mut Self::Output {
//         self.get_mut(&index).unwrap()
//     }
//  }


#[test]
fn test_linked_symbol_table() {
    let mut st: LinkedST<char,usize> = ST::new();
    assert!(st.is_empty());
    for (i, c) in "SEARCHEXAMPLE".chars().enumerate() {
        st.put(c, i);
    }

    assert_eq!(st.get(&'X'), Some(&7));
    assert_eq!(st.get(&'E'), Some(&12));
    assert_eq!(st['E'], 12);
    assert_eq!(st.size(), 10);
    // st['Z'] = 233;
    assert_eq!(st.is_empty(), false);
}
