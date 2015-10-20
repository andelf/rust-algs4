//use super::super::balanced_search_trees::RedBlackBST;
use std::cmp::Ordering;
use std::fmt;
use std::iter;
// use std::vec::IntoIter;
use super::super::searching::ST;


pub struct Node<K: PartialOrd + Copy, V> {
    pub lo: K,
    pub hi: K,
    pub max: K,
    pub val: V,
    pub left:  Option<Box<Node<K, V>>>,
    pub right: Option<Box<Node<K, V>>>
}

impl<K: PartialOrd + Copy, V> Node<K, V> {
    pub fn new(lo: K, hi: K, val: V) -> Node<K, V> {
        Node {
            lo: lo,
            hi: hi,
            max: hi,
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

    fn intersects(&self, lo: K, hi: K) -> bool {
        !(self.hi < lo || self.lo > hi)
    }
}

impl<K: fmt::Debug + PartialOrd + Copy, V: fmt::Debug> Node<K, V> {
    // FIXME: do not show .val field
    fn dump(&self, depth: usize, f: &mut fmt::Formatter, symbol: char) {
        if depth == 0 {
            writeln!(f, "({:?},{:?}) -> {:?}", self.lo, self.hi, self.max).unwrap();
        } else {
            writeln!(f, "{}{}--({:?},{:?}) -> {:?}",
                     iter::repeat("|  ").take(depth-1).collect::<Vec<&str>>().concat(),
                     symbol, self.lo, self.hi, self.max).unwrap();
        }
        if self.left.is_some() {
            self.left.as_ref().unwrap().dump(depth + 1, f, '+');
        }
        if self.right.is_some() {
            self.right.as_ref().unwrap().dump(depth + 1, f, '`');
        }
    }
}

fn put<K: PartialOrd + Copy, V>(x: Option<Box<Node<K,V>>>, lo: K, hi: K, val: V) -> Option<Box<Node<K,V>>> {
    let mut x = x;
    if x.is_none() {
        return Some(Box::new(Node::new(lo, hi, val)));
    }
    let mut cmp = lo.partial_cmp(&x.as_ref().unwrap().lo).unwrap();
    // if left node equal, use right
    if cmp == Ordering::Equal {
        cmp = hi.partial_cmp(&x.as_ref().unwrap().hi).unwrap();
    }
    match cmp {
        Ordering::Less => {
            let left = x.as_mut().unwrap().left.take();
            x.as_mut().unwrap().left = put(left, lo, hi, val);
        },
        Ordering::Greater => {
            let right = x.as_mut().unwrap().right.take();
            x.as_mut().unwrap().right = put(right, lo, hi, val);
        },
        Ordering::Equal => {
            x.as_mut().unwrap().val = val
        }
    }
    if hi > x.as_ref().unwrap().max {
        x.as_mut().unwrap().max = hi;
    }
    x
}


fn delete<K: PartialOrd + Copy, V>(x: Option<Box<Node<K,V>>>, lo: K, hi: K) -> Option<Box<Node<K,V>>> {
    let mut x = x;
    if x.is_none() {
        return None;
    }
    let mut cmp = lo.partial_cmp(&x.as_ref().unwrap().lo).unwrap();
    // if left node equal, use right
    if cmp == Ordering::Equal {
        cmp = hi.partial_cmp(&x.as_ref().unwrap().hi).unwrap();
    }
    match cmp {
        Ordering::Less => {
            let left = x.as_mut().unwrap().left.take();
            x.as_mut().unwrap().left = delete(left, lo, hi);
            return x;
        },
        Ordering::Greater => {
            let right = x.as_mut().unwrap().right.take();
            x.as_mut().unwrap().right = delete(right, lo, hi);
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


pub struct IntervalST<K: PartialOrd + Copy, V> {
    pub root: Option<Box<Node<K, V>>>
}

// IntervalST API
impl<K: PartialOrd + Copy, V> IntervalST<K, V> {
    pub fn new() -> IntervalST<K, V> {
        IntervalST { root: None }
    }

    pub fn get(&self, lo: K) -> Option<&V> {
        let mut x = self.root.as_ref();
        while x.is_some() {
            match lo.partial_cmp(&x.unwrap().lo).unwrap() {
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

    pub fn put(&mut self, lo: K, hi: K, val: V) {
        self.root = put(self.root.take(), lo, hi, val);
    }

    pub fn delete(&mut self, lo: K, hi: K) {
        self.root = delete(self.root.take(), lo, hi);
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// number of (lo, hi)-value pairs in the table
    pub fn size(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            self.root.as_ref().unwrap().size()
        }
    }

    pub fn find_intersects(&self, lo: K, hi: K) -> Option<(K, K)> {
        let mut x = self.root.as_ref();
        while x.is_some() {
            if x.unwrap().intersects(lo, hi) {
                return Some((x.unwrap().lo, x.unwrap().hi))
            } else if x.unwrap().left.is_none() {
                x = x.unwrap().right.as_ref();
            } else if x.unwrap().left.as_ref().unwrap().max < lo {
                x = x.unwrap().right.as_ref();
            } else {
                x = x.unwrap().left.as_ref();
            }
        }
        None
    }

    // pub fn intersects(&self, lo: K, hi: K) -> IntoIter<&V> {
    //     let mut result = Vec::new();
    //     let x = self.root.as_ref();
    //     while x.is_some() {
    //         if x.unwrap().intersects(lo, hi) {
    //             result.push(&x.unwrap().val)
    //         }
    //         break;
    //     }
    //     result.into_iter()
    // }
}

// delete_min helper
// returns: top, deleted
fn delete_min<K: PartialOrd + Copy, V>(x: Option<Box<Node<K,V>>>) -> (Option<Box<Node<K,V>>>, Option<Box<Node<K,V>>>) {
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



impl<K: PartialOrd + Copy, V> IntervalST<K, V> {
    pub fn keys<'a>(&'a self) -> ::std::vec::IntoIter<&'a K> {
        let mut queue: Vec<&'a K> = Vec::new();
        fn inorder<'a, K: PartialOrd + Copy, V>(x: Option<&'a Box<Node<K,V>>>, queue: &mut Vec<&'a K>) {
            if x.is_none() {
                return;
            }
            inorder(x.unwrap().left.as_ref(), queue);
            queue.push(&x.unwrap().lo);
            inorder(x.unwrap().right.as_ref(), queue);
        };
        inorder(self.root.as_ref(), &mut queue);
        queue.into_iter()
    }
}


impl<K: fmt::Debug + PartialOrd + Copy, V: fmt::Debug> fmt::Debug for IntervalST<K, V> {
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
fn test_1d_interval_search_tree() {
    let mut t = IntervalST::<i32, ()>::new();

    t.put(17, 19, ());
    t.put(5, 8, ());
    t.put(21, 24, ());
    t.put(4, 8, ());
    t.put(15, 18, ());
    t.put(7, 10, ());

    assert_eq!(t.root.as_ref().unwrap().max, 24);
    t.put(16, 22, ());
    assert_eq!(t.root.as_ref().unwrap().max, 24);

    assert_eq!((16, 22), t.find_intersects(21, 23).unwrap());
}
