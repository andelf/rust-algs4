pub struct Node<V> {
    c: char,
    left:  Option<Box<Node<V>>>,
    mid:   Option<Box<Node<V>>>,
    right: Option<Box<Node<V>>>,
    val: Option<V>,
}

impl<V> Node<V> {
    fn new(c: char) -> Node<V> {
        Node {
            c: c,
            left: None,
            mid: None,
            right: None,
            val: None
        }
    }

    fn put(mut x: Option<Box<Node<V>>>, key: &str, val: V, d: usize) -> (Option<Box<Node<V>>>, Option<V>) {
        let c = key.char_at(d);
        let replaced;
        if x.is_none() {
            x = Some(Box::new(Node::new(c)));
        }
        let xc = x.as_ref().unwrap().c;
        if c < xc {
            let (left, rplcd) = Node::put(x.as_mut().unwrap().left.take(), key, val, d);
            x.as_mut().map(|n| n.left = left);
            replaced = rplcd;
        } else if c > xc {
            let (right, rplcd) = Node::put(x.as_mut().unwrap().right.take(), key, val, d);
            x.as_mut().map(|n| n.right = right);
            replaced = rplcd;
        } else if d < key.len() - 1 {
            let (mid, rplcd) = Node::put(x.as_mut().unwrap().mid.take(), key, val, d+1);
            x.as_mut().map(|n| n.mid = mid);
            replaced = rplcd;
        } else {
            replaced = x.as_mut().unwrap().val.take();
            x.as_mut().map(|n| n.val = Some(val));
        }
        (x, replaced)
    }
}

/// Symbol table with string keys, implemented using a ternary search trie (TST).
pub struct TernarySearchTrie<V> {
    root: Option<Box<Node<V>>>,
    n: usize
}

impl<V> TernarySearchTrie<V>  {
    pub fn new() -> TernarySearchTrie<V> {
        TernarySearchTrie { root: None, n: 0 }
    }

    pub fn put(&mut self, key: &str, val: V) {
        let (root, replaced) = Node::put(self.root.take(), key, val, 0);
        self.root = root;
        // replace old val? or insert new?
        if replaced.is_none() {
            self.n += 1;
        }
    }

    pub fn get(&self, key: &str) -> Option<&V> {
        unimplemented!()
    }

    pub fn delete(&mut self, key: &str) {
        unimplemented!()
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn contains(&self, key: &str) -> bool {
        self.get(key).is_some()
    }
}


#[test]
fn test_tst() {
    let mut t = TernarySearchTrie::new();
    assert_eq!(t.size(), 0);
    t.put("name", "Andelf");
    assert_eq!(t.size(), 1);
    t.put("name", "Fledna");
    assert_eq!(t.size(), 1);
    t.put("language", "Rust");
    assert_eq!(t.size(), 2);
}
