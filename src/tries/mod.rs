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
        x.as_mut().map(|n| n.next[c] = Node::put(n.next[c].take(), key, val, d+1).take());
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

    pub fn size(&self) -> usize {
        self.n
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
}
