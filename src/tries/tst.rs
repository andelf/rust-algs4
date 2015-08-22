use adivon::queue::Queue;

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

    fn put(mut x: Option<Box<Node<V>>>, key: &str, val: Option<V>, d: usize) -> (Option<Box<Node<V>>>, Option<V>) {
        let replaced;
        let c = key.char_at(d);
        if x.is_none() {
            if val.is_none() {  // no need to call put further
                return (x, None);
            }
            x = Some(Box::new(Node::new(c)));
        }
        let xc = x.as_ref().unwrap().c;
        if c < xc {
            let (left, repl) = Node::put(x.as_mut().unwrap().left.take(), key, val, d);
            x.as_mut().map(|n| n.left = left);
            replaced = repl;
        } else if c > xc {
            let (right, repl) = Node::put(x.as_mut().unwrap().right.take(), key, val, d);
            x.as_mut().map(|n| n.right = right);
            replaced = repl;
        } else if d < key.len()-1 {
            let (mid, repl) = Node::put(x.as_mut().unwrap().mid.take(), key, val, d+1);
            x.as_mut().map(|n| n.mid = mid);
            replaced = repl;
        } else {
            replaced = x.as_mut().unwrap().val.take();
            x.as_mut().map(|n| n.val = val);
        }
        (x, replaced)
    }

    fn get<'a>(x: Option<&'a Box<Node<V>>>, key: &str, d: usize) -> Option<&'a Box<Node<V>>> {
        if x.is_none() {
            return None;
        }
        let c = key.char_at(d);
        let xc = x.unwrap().c;
        if c < xc {
            Node::get(x.unwrap().left.as_ref(), key, d)
        } else if c > xc {
            Node::get(x.unwrap().right.as_ref(), key, d)
        } else if d < key.len()-1 {
            Node::get(x.unwrap().mid.as_ref(), key, d+1)
        } else {
            x
        }
    }

    fn collect(x: Option<&Box<Node<V>>>, mut prefix: String, queue: &mut Queue<String>) {
        if x.is_none() {
            return;
        }
        Node::collect(x.unwrap().left.as_ref(), prefix.clone(), queue);
        let xc = x.unwrap().c;
        prefix.push(xc);
        if x.unwrap().val.is_some() {
            queue.enqueue(prefix.clone());
        }
        Node::collect(x.unwrap().mid.as_ref(), prefix.clone(), queue);
        prefix.pop();
        Node::collect(x.unwrap().right.as_ref(), prefix, queue);
    }

    fn longest_prefix_of<'a>(mut x: Option<&Box<Node<V>>>, query: &'a str) -> Option<&'a str> {
        let mut length = 0;
        let mut i = 0;
        while x.is_some() && i < query.len() {
            let c = query.char_at(i);
            let xc = x.unwrap().c;
            if c < xc {
                x = x.unwrap().left.as_ref();
            } else if c > xc {
                x = x.unwrap().right.as_ref();
            } else {
                i += 1;
                if x.unwrap().val.is_some() {
                    length = i;
                }
                x = x.unwrap().mid.as_ref();
            }
        }
        if length == 0 {
            None
        } else {
            Some(&query[..length])
        }
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
        let (root, replaced) = Node::put(self.root.take(), key, Some(val), 0);
        self.root = root;
        // replace old val? or insert new?
        if replaced.is_none() {
            self.n += 1;
        }
    }

    pub fn get(&self, key: &str) -> Option<&V> {
        assert!(key.len() > 0, "key must have length >= 1");
        Node::get(self.root.as_ref(), key, 0).map_or(None, |n| n.val.as_ref())
    }

    pub fn delete(&mut self, key: &str) {
        let (root, replaced) = Node::put(self.root.take(), key, None, 0);
        self.root = root;
        // deleted?
        if replaced.is_some() {
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
        self.get(key).is_some()
    }

    pub fn longest_prefix_of<'a>(&self, query: &'a str) -> Option<&'a str> {
        Node::longest_prefix_of(self.root.as_ref(), query)
    }

    pub fn keys_with_prefix(&self, prefix: &str) -> Vec<String> {
        let mut queue = Queue::new();
        let x = Node::get(self.root.as_ref(), prefix, 0);
        if x.is_some() {
            if x.unwrap().val.is_some() {
                queue.enqueue(prefix.into());
            }
            Node::collect(x.unwrap().mid.as_ref(), prefix.into(), &mut queue);
        }
        queue.into_iter().collect()
    }

    pub fn keys(&self) -> Vec<String> {
        let mut queue = Queue::new();
        Node::collect(self.root.as_ref(), "".into(), &mut queue);
        queue.into_iter().collect()
    }
}


#[test]
fn test_tst() {
    let mut t = TernarySearchTrie::new();
    assert_eq!(t.size(), 0);
    t.put("name", "Andelf");
    assert_eq!(t.size(), 1);
    assert!(t.contains("name"));
    t.put("name", "Fledna");
    assert_eq!(t.size(), 1);
    t.put("language", "Rust");
    assert_eq!(t.size(), 2);

    assert_eq!(t.get("name"), Some(&"Fledna"));
    assert_eq!(t.get("whatever"), None);

    t.delete("name");
    assert_eq!(t.size(), 1);
    assert_eq!(t.get("name"), None);

    t.put("name", "Lednaf");
    assert!(t.keys().contains(&"name".into()));
    assert!(t.keys().contains(&"language".into()));
    assert!(t.keys_with_prefix("lang").contains(&"language".into()));

    t.put("ban", "2333");
    t.put("banana", "2333");
    assert_eq!(t.longest_prefix_of("bananananana"), Some("banana"));
}
