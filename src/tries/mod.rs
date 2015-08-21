use std::char;
use std::usize;
use adivon::queue::Queue;

const R: usize = 256;

/// 245-Way trie
pub struct Node<V> {
    val: Option<V>,
    next: Vec<Option<Node<V>>>
}

impl<V> Node<V> {
    fn new() -> Node<V> {
        Node {
            val: None,
            next: {
                let mut next = Vec::with_capacity(R);
                for _ in 0 .. R { next.push(None) }
                next
            }
        }
    }

    fn put(mut x: Option<Node<V>>, key: &str, val: V, d: usize) -> Option<Node<V>> {
        if x.is_none() {
            x = Some(Node::new());
        }
        if d == key.bytes().len() {
            x.as_mut().map(|n| n.val = Some(val));
            return x
        }
        let c = key.as_bytes()[d] as usize;
        x.as_mut().map(|n| n.next[c] = Node::put(n.next[c].take(), key, val, d+1));
        x
    }

    fn get<'a>(x: Option<&'a Node<V>>, key: &str, d: usize) -> Option<&'a Node<V>> {
        if x.is_none() {
            None
        } else if d == key.bytes().len() {
            x
        } else {
            let c = key.as_bytes()[d] as usize;
            x.as_ref().map(|n| Node::get(n.next[c].as_ref(), key, d+1)).unwrap()
        }
    }

    fn delete(mut x: Option<Node<V>>, key: &str, d: usize) -> (Option<Node<V>>, Option<V>) {
        let mut deleted = None;
        if x.is_none() {
            return (None, deleted);
        }
        if d == key.bytes().len() {
            if x.as_ref().map_or(false, |n| n.val.is_some()) {
                deleted = x.as_mut().map(|n| n.val.take()).unwrap();
            }
        } else {
            let c = key.as_bytes()[d] as usize;
            x.as_mut().map(|n| {
                // FIXME: https://github.com/rust-lang/rfcs/issues/372
                let (nx, nd) = Node::delete(n.next[c].take(), key, d+1);
                n.next[c] = nx;
                deleted = nd;
            });
        }
        if x.as_ref().map_or(false, |n| n.val.is_some()) {
            return (x, deleted);
        }
        for c in 0 .. R {
            if x.as_ref().map_or(false, |n| n.next[c].is_some()) {
                return (x, deleted);
            }
        }
        return (None, deleted);
    }

    // use '?' as pattern
    fn collect_by_pattern(x: Option<&Node<V>>, mut prefix: String, pattern: &str, results: &mut Queue<String>) {
        if x.is_none() {
            return;
        }
        let d = prefix.bytes().len();
        if d == pattern.bytes().len() {
            if x.map_or(false, |n| n.val.is_some()) {
                results.enqueue(prefix.clone());
            }
            return;
        }
        let c = pattern.as_bytes()[d];
        if c == '?' as u8 {
            for ch in 0 .. R {
                prefix.push(char::from_u32(ch as u32).unwrap());
                Node::collect_by_pattern(x.unwrap().next[ch].as_ref(),
                                         prefix.clone(), pattern, results);
                prefix.pop();
            }
        } else {
            prefix.push(c as char);
            Node::collect_by_pattern(x.unwrap().next[c as usize].as_ref(),
                                     prefix.clone(), pattern, results);
            prefix.pop();
        }
    }

    fn collect(x: Option<&Node<V>>, mut prefix: String, results: &mut Queue<String>) {
        if x.is_none() {
            return;
        }
        if x.unwrap().val.is_some() {
            results.enqueue(prefix.clone());
        }
        for c in 0 .. R {
            prefix.push(char::from_u32(c as u32).unwrap());
            Node::collect(x.unwrap().next[c].as_ref(), prefix.clone(), results);
            prefix.pop();
        }
    }

    fn longest_prefix_of(x: Option<&Node<V>>, query: &str, d: usize, mut length: usize) -> usize {
        if x.is_none() {
            return length;
        }
        if x.unwrap().val.is_some() {
            length = d;
        }
        if d == query.len() {
            return length;
        }
        let c = query.as_bytes()[d];
        Node::longest_prefix_of(x.as_ref().unwrap().next[c as usize].as_ref(), query, d+1, length)
    }
}

pub struct TrieST<V> {
    root: Option<Node<V>>,
    n: usize
}

impl<V> TrieST<V> {
    pub fn new() -> TrieST<V> {
        TrieST {
            root: None,
            n: 0
        }
    }

    pub fn put(&mut self, key: &str, val: V) {
        self.root = Node::put(self.root.take(), key, val, 0);
        self.n += 1;
    }

    pub fn get(&self, key: &str) -> Option<&V> {
        Node::get(self.root.as_ref(), key, 0).map(|n| n.val.as_ref()).unwrap_or(None)
    }

    pub fn delete(&mut self, key: &str) {
        let (root, deleted) = Node::delete(self.root.take(), key, 0);
        self.root = root;
        if deleted.is_some() {
            self.n -= 1;
        }
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn contains(&self, key: &str) -> bool {
        Node::get(self.root.as_ref(), key, 0).map(|n| n.val.is_some()).unwrap_or(false)
    }

    pub fn keys_with_prefix(&self, prefix: &str) -> Vec<String> {
        let mut results = Queue::new();
        let x = Node::get(self.root.as_ref(), prefix, 0);
        Node::collect(x, prefix.into(), &mut results);
        results.into_iter().collect()
    }

    pub fn keys(&self) -> Vec<String> {
        self.keys_with_prefix("")
    }

    pub fn keys_that_match(&self, pattern: &str) -> Vec<String> {
        let mut results = Queue::new();
        Node::collect_by_pattern(self.root.as_ref(), "".into(), pattern, &mut results);
        results.into_iter().collect()
    }

    pub fn longest_prefix_of<'a>(&self, query: &'a str) -> Option<&'a str> {
        let length = Node::longest_prefix_of(self.root.as_ref(), query, 0, usize::MAX);
        if length == usize::MAX {
            None
        } else {
            Some(&query[..length])
        }

    }
}



#[test]
fn test_tries() {
    let mut t = TrieST::new();
    assert_eq!(t.size(), 0);
    t.put("name", "Andelf");
    t.put("tel", "110");
    t.put("addr1", "Shaanxi");
    t.put("addr long", "Beijing Haidian");
    t.put("addr2", "Beijing");
    assert_eq!(t.size(), 5);
    assert_eq!(t.get("addr2"), Some(&"Beijing"));
    assert_eq!(t.get("non-exists-key"), None);
    assert!(t.contains("tel"));
    t.delete("tel");
    assert_eq!(t.size(), 4);
    assert!(!t.contains("tel"));
    assert_eq!(vec!["addr long", "addr1", "addr2", "name"], t.keys());
    t.put("addrs", "Beijing, Tianjin, Xi'an");
    t.put("addr22", "Sanya");
    assert_eq!(t.keys_that_match("addr?").len(), 3);
    assert_eq!(t.keys_that_match("addr??"), vec!["addr22"]);
    assert_eq!(t.longest_prefix_of("addr22222"), Some("addr22"));
}

// TST
// pub struct TernarySearchTrie { }
