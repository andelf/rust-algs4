use std::iter;
use std::fmt;
use std::cmp::Ordering;
use super::{ST, OrderedST};
// FIXME: out implementation can't be used. :(
// use super::super::stacks_and_queues::Queue;
// use super::super::stacks_and_queues::resizing_array_queue::ResizingArrayQueue;

pub struct Node<K, V> {
    pub key: K,
    pub val: V,
    pub left:  Option<Box<Node<K, V>>>,
    pub right: Option<Box<Node<K, V>>>
}

impl<K, V> Node<K, V> {
    pub fn new(key: K, val: V) -> Node<K, V> {
        Node {
            key: key,
            val: val,
            left: None,
            right: None
        }
    }

    fn size(&self) -> usize {
        let mut ret = 1;
        if self.left.is_some() {
            ret += self.left.as_ref().unwrap().size()
        }
        if self.right.is_some() {
            ret += self.right.as_ref().unwrap().size()
        }
        ret
    }
}

impl<K: fmt::Debug, V: fmt::Debug> Node<K, V> {
    fn dump(&self, depth: usize, f: &mut fmt::Formatter, symbol: char) {
        if depth == 0 {
            writeln!(f, "{:?}[{:?}]", self.key, self.val).unwrap();
        } else {
            writeln!(f, "{}{}--{:?}[{:?}]", iter::repeat("|  ").take(depth-1).collect::<Vec<&str>>().concat(),
                     symbol, self.key, self.val).unwrap();
        }
        if self.left.is_some() {
            self.left.as_ref().unwrap().dump(depth + 1, f, '+');
        }
        if self.right.is_some() {
            self.right.as_ref().unwrap().dump(depth + 1, f, '`');
        }
    }
}

fn put<K: PartialOrd, V>(x: Option<Box<Node<K,V>>>, key: K, val: V) -> Option<Box<Node<K,V>>> {
    let mut x = x;
    if x.is_none() {
        return Some(Box::new(Node::new(key, val)));
    }
    let cmp = key.partial_cmp(&x.as_ref().unwrap().key).unwrap();
    match cmp {
        Ordering::Less => {
            let left = x.as_mut().unwrap().left.take();
            x.as_mut().unwrap().left = put(left, key, val)
        },
        Ordering::Greater => {
            let right = x.as_mut().unwrap().right.take();
            x.as_mut().unwrap().right = put(right, key, val)
        },
        Ordering::Equal => {
            x.as_mut().unwrap().val = val
        }
    }
    x
}


fn delete<K: PartialOrd, V>(x: Option<Box<Node<K,V>>>, key: &K) -> Option<Box<Node<K,V>>> {
    if x.is_none() {
        return None;
    }

    let mut x = x;
    match key.partial_cmp(&x.as_ref().unwrap().key).unwrap() {
        Ordering::Less => {
            let left = x.as_mut().unwrap().left.take();
            x.as_mut().unwrap().left = delete(left, key);
            return x;
        },
        Ordering::Greater => {
            let right = x.as_mut().unwrap().right.take();
            x.as_mut().unwrap().right = delete(right, key);
            return x;
        },
        Ordering::Equal => {
            if x.as_ref().unwrap().right.is_none() {
                return x.as_mut().unwrap().left.take();
            }
            if x.as_ref().unwrap().left.is_none() {
                return x.as_mut().unwrap().right.take();
            }

            // Save top
            let mut t = x;

            // split right into right without min, and the min
            let (right, right_min) = delete_min(t.as_mut().unwrap().right.take());
            x = right_min;
            x.as_mut().unwrap().right = right;
            x.as_mut().unwrap().left = t.as_mut().unwrap().left.take();
            x
        }
    }
}

pub struct BST<K, V> {
    pub root: Option<Box<Node<K, V>>>
}

impl<K: PartialOrd, V> ST<K, V> for BST<K, V> {
    fn new() -> BST<K, V> {
        BST { root: None }
    }

    fn get(&self, key: &K) -> Option<&V> {
        let mut x = self.root.as_ref();
        while x.is_some() {
            match key.partial_cmp(&x.unwrap().key).unwrap() {
                Ordering::Less => {
                    x = x.unwrap().left.as_ref();
                },
                Ordering::Greater => {
                    x = x.unwrap().right.as_ref();
                },
                Ordering::Equal  => {
                    return Some(&x.unwrap().val)
                }
            }
        }
        None
    }

    fn put(&mut self, key: K, val: V) {
        self.root = put(self.root.take(), key, val);
    }

    fn delete(&mut self, key: &K) {
        self.root = delete(self.root.take(), key);
    }

    fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// number of key-value pairs in the table
    fn size(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            self.root.as_ref().unwrap().size()
        }
    }
}

fn floor<'a, K: PartialOrd, V>(x: Option<&'a Box<Node<K,V>>>, key: &K) -> Option<&'a Node<K,V>> {
    if x.is_none() {
        return None;
    }

    match key.partial_cmp(&x.unwrap().key).unwrap() {
        Ordering::Equal => {
            return Some(&(**x.unwrap()));
        },
        Ordering::Less => {
            return floor(x.unwrap().left.as_ref(), key);
        },
        _ => (),
    }

    let t = floor(x.unwrap().right.as_ref(), key);
    if t.is_some() {
        return t
    } else {
        return Some(x.unwrap())
    }
}

fn ceiling<'a, K: PartialOrd, V>(x: Option<&'a Box<Node<K,V>>>, key: &K) -> Option<&'a Node<K,V>> {
    if x.is_none() {
        return None;
    }

    match key.partial_cmp(&x.unwrap().key).unwrap() {
        Ordering::Equal => {
            return Some(&(**x.unwrap()));
        },
        Ordering::Greater => {
            return ceiling(x.unwrap().right.as_ref(), key);
        },
        _ => (),
    }

    let t = ceiling(x.unwrap().left.as_ref(), key);
    if t.is_some() {
        return t
    } else {
        return Some(x.unwrap())
    }
}

// delete_min helper
// returns: top, deleted
fn delete_min<K: PartialOrd, V>(x: Option<Box<Node<K,V>>>) -> (Option<Box<Node<K,V>>>, Option<Box<Node<K,V>>>) {
    let mut x = x;
    if x.is_none() {
        return (None, None);
    }
    match x.as_mut().unwrap().left.take() {
        None           => (x.as_mut().unwrap().right.take(), x),
        left @ Some(_) => {
            let (t, deleted) = delete_min(left);
            x.as_mut().unwrap().left = t;
            (x, deleted)
        }
    }
}

// delete_max helper
// returns: top, deleted
fn delete_max<K: PartialOrd, V>(x: Option<Box<Node<K,V>>>) -> (Option<Box<Node<K,V>>>, Option<Box<Node<K,V>>>) {
    let mut x = x;
    if x.is_none() {
        return (None, None);
    }
    match x.as_mut().unwrap().right.take() {
        None            => (x.as_mut().unwrap().left.take(), x),
        right @ Some(_) => {
            let (t, deleted) = delete_max(right);
            x.as_mut().unwrap().right = t;
            (x, deleted)
        }
    }
}

fn find_max<K: PartialOrd, V>(x: Option<&Box<Node<K,V>>>) -> Option<&Box<Node<K,V>>> {
    if x.is_none() {
        return None;
    }
    match x.as_ref().unwrap().right.as_ref() {
        None            => x,
        right @ Some(_) => find_max(right)
    }
}

fn find_min<K: PartialOrd, V>(x: Option<&Box<Node<K,V>>>) -> Option<&Box<Node<K,V>>> {
    if x.is_none() {
        return None;
    }
    match x.as_ref().unwrap().left.as_ref() {
        None           => x,
        left @ Some(_) => find_min(left)
    }
}

impl<K: PartialOrd, V> OrderedST<K, V> for BST<K, V> {
    /// smallest key
    fn min(&self) -> Option<&K> {
        find_min(self.root.as_ref()).map(|n| &n.key)
    }

    /// largest key
    fn max(&self) -> Option<&K> {
        find_max(self.root.as_ref()).map(|n| &n.key)
    }

    /// largest key less than or equal to key
    fn floor(&self, key: &K) -> Option<&K> {
        let x = floor(self.root.as_ref(), key);
        if x.is_none() {
            None
        } else {
            Some(&x.unwrap().key)
        }
    }

    /// smallest key greater than or equal to key
    fn ceiling(&self, key: &K) -> Option<&K> {
        let x = ceiling(self.root.as_ref(), key);
        if x.is_none() {
            None
        } else {
            Some(&x.unwrap().key)
        }
    }

    /// number of keys less than key
    fn rank(&self, key: &K) -> usize {
        fn rank_helper<'a, K: PartialOrd, V>(x: Option<&'a Box<Node<K,V>>>, key: &K) -> usize {
            if x.is_none() {
                return 0;
            }

            match key.partial_cmp(&x.unwrap().key).unwrap() {
                Ordering::Less => {
                    rank_helper(x.unwrap().left.as_ref(), key)
                },
                Ordering::Greater => {
                    1 + x.as_ref().unwrap().left.as_ref().map(|ref n| n.size()).unwrap_or(0) +
                        rank_helper(x.unwrap().right.as_ref(), key)
                }
                Ordering::Equal => {
                    x.as_ref().unwrap().left.as_ref().map(|ref n| n.size()).unwrap_or(0)
                }
            }
        }

        rank_helper(self.root.as_ref(), key)
    }

    /// key of rank k
    fn select(&self, k: usize) -> Option<&K> {
        for key in self.keys() {
            if self.rank(key) == k {
                return Some(key)
            }
        }
        None
    }

    /// delete smallest key
    fn delete_min(&mut self) {
        self.root = delete_min(self.root.take()).0;
    }

    /// delete largest key
    fn delete_max(&mut self) {
        self.root = delete_max(self.root.take()).0;
    }
}


impl<K: PartialOrd, V> BST<K, V> {
    pub fn keys<'a>(&'a self) -> ::std::vec::IntoIter<&'a K> {
        let mut queue: Vec<&'a K> = Vec::new();
        fn inorder<'a, K, V>(x: Option<&'a Box<Node<K,V>>>, queue: &mut Vec<&'a K>) {
            if x.is_none() {
                return;
            }
            inorder(x.unwrap().left.as_ref(), queue);
            queue.push(&x.unwrap().key);
            inorder(x.unwrap().right.as_ref(), queue);
        };
        inorder(self.root.as_ref(), &mut queue);
        queue.into_iter()
    }
}


impl<K: fmt::Debug, V: fmt::Debug> fmt::Debug for BST<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.root.is_none() {
            write!(f, "<empty tree>")
        } else {
            self.root.as_ref().unwrap().dump(0, f, ' ');
            Ok(())
        }
    }
}

#[test]
fn test_binary_search_tree_delete_min() {
    let mut t = BST::<char, ()>::new();
    for c in "SEARCHEXAMP".chars() {
        t.put(c, ());
    }

    let sz0 = t.size();
    assert!(t.get(&'A').is_some());
    t.delete_min();             // assume delete 'A'
    assert_eq!(t.get(&'A'), None);
    assert_eq!(t.size(), sz0 - 1);
}

#[test]
fn test_binary_search_tree_delete_max() {
    let mut t = BST::<char, ()>::new();
    for c in "SEARCHEXAMP".chars() {
        t.put(c, ());
    }

    let sz0 = t.size();
    assert!(t.get(&'X').is_some());
    t.delete_max();             // assume delete 'X'
    assert_eq!(t.get(&'X'), None);
    assert_eq!(t.size(), sz0 - 1);
}

#[test]
fn test_binary_search_tree_delete_case0() {
    let mut t = BST::<char, ()>::new();
    for c in "SEARCHEXAM".chars() {
        t.put(c, ());
    }

    let sz0 = t.size();
    t.delete(&'C');
    assert_eq!(t.get(&'C'), None);
    assert_eq!(t.size(), sz0 - 1);
}

#[test]
fn test_binary_search_tree_delete_case1() {
    let mut t = BST::<char, ()>::new();
    for c in "SEARCHEXAM".chars() {
        t.put(c, ());
    }

    let sz0 = t.size();
    t.delete(&'R');
    assert_eq!(t.get(&'R'), None);
    assert_eq!(t.size(), sz0 - 1);
}

#[test]
fn test_binary_search_tree_delete_case2() {
    let mut t = BST::<char, ()>::new();
    for c in "SEARCHEXAM".chars() {
        t.put(c, ());
    }

    let sz0 = t.size();
    t.delete(&'E');
    assert_eq!(t.get(&'E'), None);
    assert_eq!(t.size(), sz0 - 1);
}

#[test]
fn test_binary_search_tree() {
    use std::iter::FromIterator;

    let mut t = BST::<char, usize>::new();
    for (i, c) in "SEARCHEXAMP".chars().enumerate() {
        t.put(c, i);
    }

    // println!("{:?}", t);
    assert_eq!(t.get(&'E'),  Some(&6));
    assert_eq!(t.floor(&'O'), Some(&'M'));
    assert_eq!(t.ceiling(&'Q'), Some(&'R'));
    assert_eq!(t.size(), 9);
    assert_eq!(t.rank(&'E'), 2);
    assert_eq!(t.select(2), Some(&'E'));
    assert_eq!(t.rank(&'M'), 4);
    assert_eq!(t.select(4), Some(&'M'));
    assert_eq!(t.max(), Some(&'X'));
    assert_eq!(t.min(), Some(&'A'));
    // inorder visite
    assert_eq!(String::from_iter(t.keys().map(|&c| c)), "ACEHMPRSX");
}
