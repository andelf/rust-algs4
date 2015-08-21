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
}

/// Symbol table with string keys, implemented using a ternary search trie (TST).
pub struct TernarySearchTrie<V> {
    root: Option<Node<V>>,
    n: usize
}

impl<V> TernarySearchTrie<V>  {
    pub fn new() -> TernarySearchTrie<V> {
        TernarySearchTrie { root: None, n: 0 }
    }

    pub fn put(&mut self, key: &str, val: V) {
        unimplemented!()
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
