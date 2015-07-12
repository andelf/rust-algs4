use std::iter;
use std::fmt;
use std::cmp::Ordering;
// pub trait BST<K: Ord, V> {
//     fn put(&mut self, key: K, val: V);
//     fn get(&self, key: K) -> Option<&V>;
//     fn delete(&mut self, key: &key);
//     // fn keys()
// }



pub struct Node<K, V> {
    key: K,
    val: V,
    left:  Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>
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
}

impl<K: fmt::Debug, V: fmt::Debug> Node<K, V> {
    fn dump(&self, depth: usize, f: &mut fmt::Formatter) {
        if depth == 0 {
            writeln!(f, "{:?}[{:?}]", self.key, self.val);
        } else {
            writeln!(f, "{}+--{:?}[{:?}]", iter::repeat("|  ").take(depth-1).collect::<Vec<&str>>().concat(),
                     self.key, self.val);
        }
        if self.left.is_some() {
            self.left.as_ref().unwrap().dump(depth + 1, f);
        }
        if self.right.is_some() {
            self.right.as_ref().unwrap().dump(depth + 1, f);
        }
    }
}

fn put<K: Ord, V>(x: Option<Box<Node<K,V>>>, key: K, val: V) -> Option<Box<Node<K,V>>> {
    let mut x = x;
    if x.is_none() {
        return Some(Box::new(Node::new(key, val)));
    }
    let cmp = key.cmp(&x.as_ref().unwrap().key);
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


pub struct BST<K, V> {
    root: Option<Box<Node<K, V>>>
}

impl<K: Ord, V> BST<K, V> {
    pub fn new() -> BST<K, V> {
        BST { root: None }
    }

    pub fn get(&self, key: K) -> Option<&V> {
        let mut x = self.root.as_ref();
        while x.is_some() {
            match key.cmp(&x.unwrap().key) {
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

    pub fn put(&mut self, key: K, val: V) {
        let root = self.root.take();
        self.root = put(root, key, val);
        // if self.root.is_none() {
        //     self.root = Some(Box::new(Node::new(key, val)))
        // } else {

        // }
    }
//     fn delete(&mut self, key: &key);
}

impl<K: fmt::Debug, V: fmt::Debug> fmt::Debug for BST<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.root.is_none() {
            write!(f, "<empty tree>")
        } else {
            self.root.as_ref().unwrap().dump(0, f);
            Ok(())
        }
    }
}



#[test]
fn test_binary_search_tree() {
    let mut t = BST::<char, usize>::new();
    for (i, c) in "SEARCHEXAMP".chars().enumerate() {
        t.put(c, i);
    }

    //println!("{:?}", t);
    assert_eq!(t.get('E'),  Some(&6));
}
