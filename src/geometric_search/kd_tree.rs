use super::primitive::{Point2D, RectHV};

use std::iter;
use std::fmt;
use std::cmp::Ordering;
use super::super::symbol_tables::ST;


pub trait Point: Copy {
    type ValueType: Copy + PartialOrd = f64;
    // const DIMENSION: usize = 2;

    #[inline]
    fn get(&self, d: usize) -> Self::ValueType;

    #[inline]
    fn dimension() -> usize { 2 }
}

impl Point for Point2D {
    #[inline]
    fn get(&self, d: usize) -> f64 {
        if d == 0 {
            self.x
        } else if d == 1 {
            self.y
        } else {
            panic!("dimension not supported")
        }
    }
}

pub struct Node<K: Point, V> {
    pub key: K,
    pub val: V,
    pub left:  Option<Box<Node<K, V>>>,
    pub right: Option<Box<Node<K, V>>>,
    pub depth: usize
}

impl<K: Point, V> Node<K, V> {
    pub fn new(key: K, val: V, depth: usize) -> Node<K, V> {
        Node {
            key: key,
            val: val,
            left: None,
            right: None,
            // depth use (depth % k)-th dimension
            depth: depth
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

impl<K: Point + fmt::Debug, V: fmt::Debug> Node<K, V> {
    fn dump(&self, depth: usize, f: &mut fmt::Formatter, symbol: char) {
        if depth == 0 {
            writeln!(f, "\n{:?}[{:?}]", self.key, self.val).unwrap();
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

fn put<K: Point, V>(x: Option<Box<Node<K,V>>>, key: K, val: V, depth: usize) -> Option<Box<Node<K,V>>> {
    let mut x = x;
    if x.is_none() {
        return Some(Box::new(Node::new(key, val, depth)));
    }
    let depth = x.as_ref().unwrap().depth;
    let dim = x.as_ref().unwrap().depth % <K as Point>::dimension();

    let cmp = key.get(dim).partial_cmp(&x.as_ref().unwrap().key.get(dim)).unwrap();
    match cmp {
        Ordering::Less => {
            let left = x.as_mut().unwrap().left.take();
            x.as_mut().unwrap().left = put(left, key, val, depth + 1)
        },
        Ordering::Greater => {
            let right = x.as_mut().unwrap().right.take();
            x.as_mut().unwrap().right = put(right, key, val, depth + 1)
        },
        Ordering::Equal => {
            x.as_mut().unwrap().val = val
        }
    }
    x
}

fn delete_min<K: Point, V>(x: Option<Box<Node<K,V>>>) -> (Option<Box<Node<K,V>>>, Option<Box<Node<K,V>>>) {
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

fn delete<K: Point, V>(x: Option<Box<Node<K,V>>>, key: &K) -> Option<Box<Node<K,V>>> {
    if x.is_none() {
        return None;
    }

    let mut x = x;
    let dim = x.as_ref().unwrap().depth % <K as Point>::dimension();

    match key.get(dim).partial_cmp(&x.as_ref().unwrap().key.get(dim)).unwrap() {
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

pub struct KdTree<K: Point, V> {
    pub root: Option<Box<Node<K, V>>>
}

impl<K: Point, V> ST<K, V> for KdTree<K, V> {
    fn new() -> KdTree<K, V> {
        KdTree { root: None }
    }

    fn get(&self, key: &K) -> Option<&V> {
        let mut x = self.root.as_ref();
        while x.is_some() {
            let dim = x.as_ref().unwrap().depth % <K as Point>::dimension();
            match key.get(dim).partial_cmp(&x.unwrap().key.get(dim)).unwrap() {
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
        self.root = put(self.root.take(), key, val, 0);
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


impl<K: Point, V> KdTree<K, V> {
    pub fn keys<'a>(&'a self) -> ::std::vec::IntoIter<&'a K> {
        let mut queue: Vec<&'a K> = Vec::new();
        fn inorder<'a, K: Point, V>(x: Option<&'a Box<Node<K,V>>>, queue: &mut Vec<&'a K>) {
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

impl<V> KdTree<Point2D, V> {
    /// find all Point2D keys that lie in a 2d range
    pub fn range_search<T: AsRef<RectHV>>(&self, rect: T) -> Option<&Point2D> {
        unimplemented!()
    }

    /// number of keys that lie in a 2d range
    pub fn range_count<T: AsRef<RectHV>>(&self, rect: T) -> usize {
        unimplemented!()
    }
}


impl<K: Point + fmt::Debug, V: fmt::Debug> fmt::Debug for KdTree<K, V> {
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
fn test_kd_tree_with_point_2d() {
    let mut t = KdTree::<Point2D, ()>::new();

    t.put(Point2D::new(0.7, 0.2), ());
    t.put(Point2D::new(0.5, 0.4), ());
    t.put(Point2D::new(0.2, 0.3), ());
    t.put(Point2D::new(0.4, 0.7), ());
    t.put(Point2D::new(0.9, 0.6), ());

    println!("got => {:?}", t);
}
