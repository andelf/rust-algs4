use std::ops::Index;
use std::cmp::Ordering;
use super::ST;

pub struct OrderedArrayST<K, V> {
    // Maintain an ordered array of key-value pairs.
    pairs: Vec<(K,V)>,
}

impl<K: PartialOrd, V> OrderedArrayST<K, V> {
    fn rank(&self, key: &K) -> usize {
        let mut lo = 0;
        let mut hi = self.pairs.len() - 1;

        while lo <= hi {
            let mid = lo + (hi - lo)/2;
            if mid == 0 {
                break;
            }
            match key.partial_cmp(&self.pairs[mid].0).unwrap() {
                Ordering::Less => {
                    hi = mid - 1;
                },
                Ordering::Greater => {
                    lo = mid + 1;
                },
                _ => {
                    return mid;
                }
            }
        }
        lo
    }

    fn insertion_sort_by_key(&mut self) {
        let n = self.pairs.len();
        for i in 0 .. n {
            for j in (0 .. i).rev() {
                if self.pairs[j + 1].0 < self.pairs[j].0 {
                    self.pairs.swap(j+1, j);
                } else {
                    break;
                }
            }
        }
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if self.is_empty() {
            return None;
        }
        let i = self.rank(key);
        if i < self.pairs.len() && &self.pairs[i].0 == key {
            Some(&mut self.pairs[i].1)
        } else {
            None
        }
    }
}

impl<K: PartialOrd, V> ST<K,V> for OrderedArrayST<K, V> {
    fn new() -> Self {
        OrderedArrayST {
            pairs: Vec::new(),
        }
    }

    fn put(&mut self, key: K, val: V) {
        if let Some(v) = self.get_mut(&key) {
            *v = val;
            return;
        }
        self.pairs.push((key, val));
        self.insertion_sort_by_key();
    }

    fn get(&self, key: &K) -> Option<&V> {
        if self.is_empty() {
            return None;
        }
        let i = self.rank(key);
        if i < self.pairs.len() && &self.pairs[i].0 == key {
            Some(&self.pairs[i].1)
        } else {
            None
        }
    }

    fn delete(&mut self, key: &K) {
        let i = self.rank(key);
        if i < self.pairs.len() && &self.pairs[i].0 == key {
            self.pairs.remove(i);
        }
    }

    /// is the table empty?
    fn is_empty(&self) -> bool {
        self.pairs.len() == 0
    }

    fn size(&self) -> usize {
        self.pairs.len()
    }
}



impl<K: Ord, V> Index<K> for OrderedArrayST<K, V> {
    type Output = V;

    fn index<'a>(&'a self, index: K) -> &'a Self::Output {
        self.get(&index).unwrap()
    }
}


#[test]
fn test_ordered_array_symbol_table() {
    let mut st: OrderedArrayST<char,usize> = ST::new();
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
