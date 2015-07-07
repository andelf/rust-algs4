use std::mem;
use std::ptr;
use std::fmt;
use std::iter::{Iterator, ExactSizeIterator};
use super::Deque;

struct Rawlink<T> {
    p: *mut T
}

impl<T> Rawlink<T> {
    fn none() -> Rawlink<T> {
        Rawlink { p: ptr::null_mut() }
    }

    fn some(n: &mut T) -> Rawlink<T> {
        Rawlink { p: n }
    }

    fn take(&mut self) -> Rawlink<T> {
        mem::replace(self, Rawlink::none())
    }
}

struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>,
    prev: Rawlink<Node<T>>

}

impl<T> Node<T> {
    /// work around for moved value
    fn into_item_and_pointers(self) -> (T, Option<Box<Node<T>>>, Rawlink<Node<T>>) {
        (self.item, self.next, self.prev)
    }

    fn size(&self) -> usize {
        let mut p = self.next.as_ref();
        let mut sz = 1;
        while p.is_some() {
            p = p.unwrap().next.as_ref();
            sz += 1;
        }
        sz
    }
}

/// linked double queue
pub struct LinkedDeque<T> {
    first: Option<Box<Node<T>>>,
    last: Rawlink<Node<T>>
}

impl<T> Deque<T> for LinkedDeque<T> {
    fn new() -> LinkedDeque<T> {
        LinkedDeque {
            first: None,
            last: Rawlink::none()
        }
    }
    fn is_empty(&self) -> bool {
        self.first.is_none()
    }
    fn size(&self) -> usize {
        match self.first {
            None    => 0,
            Some(ref l) => l.size()
        }
    }
    fn add_first(&mut self, item: T) {
        let mut old_first = self.first.take();
        let mut first = Box::new(Node {
            item: item,
            next: None,
            prev: Rawlink::none()
        });

        if old_first.is_some() {
            old_first.as_mut().unwrap().prev = Rawlink::some(&mut first);
            // move in
            first.next = old_first;
        } else {
            self.last = Rawlink::some(&mut first);
        }

        self.first = Some(first)
    }

    fn add_last(&mut self, item: T) {
        if self.first.is_some() {
            let old_last = self.last.take();
            let mut last = Box::new(Node {
                item: item,
                next: None,
                prev: Rawlink::none(),
            });
            self.last = Rawlink::some(&mut last);
            unsafe {
                (*old_last.p).next = Some(last);
            }

        } else {
            self.add_first(item)
        }
    }

    fn remove_first(&mut self) -> Option<T> {
        let old_first = self.first.take();
        if old_first.is_some() {
            let (item, mut first, _) = old_first.unwrap().into_item_and_pointers();
            // update new first's prev field
            first.as_mut().map(|v| v.prev = Rawlink::none());
            self.first = first;
            Some(item)
        } else {
            None
        }
    }

    fn remove_last(&mut self) -> Option<T> {
        let old_last = self.last.take();
        if old_last.p.is_null() {
            return None;
        }
        let last_ref_mut = unsafe { mem::transmute::<_, &mut Node<T>>(old_last.p) };

        let last: Node<T> = mem::replace(last_ref_mut, unsafe { mem::zeroed() });

        if last.prev.p.is_null() {
            self.first = None;
        } else {
            unsafe {
                (*last.prev.p).next = None;
            }
        }
        self.last = last.prev;

        Some(last.item)
    }

}

impl<T> LinkedDeque<T> {
    fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            current: self.first.as_ref(),
            nelem: self.size()
        }
    }
}

impl<T: fmt::Display> fmt::Display for LinkedDeque<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.first {
            None    => {
                try!(write!(f, "<empty deque>"));
            },
            Some(ref l) => {
                try!(write!(f, "("));
                let mut p = Some(l);
                while p.is_some() {
                    try!(write!(f, "{},", p.unwrap().item));
                    p = p.unwrap().next.as_ref();
                }
                try!(write!(f, ")"));

            }
        }
        Ok(())
    }
}

// TODO impl DoubleEndedIterator, ExactSizeIterator
pub struct Iter<'a, T: 'a> {
    current: Option<&'a Box<Node<T>>>,
    nelem: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.nelem == 0 {
            return None;
        }
        let old_current = self.current.take();

        self.current = (**old_current.unwrap()).next.as_ref();
        self.nelem -= 1;
        Some(&old_current.as_ref().unwrap().item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.nelem, Some(self.nelem))
    }
}

impl <'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.nelem
    }
}


#[test]
fn test_linked_deque_add_remove() {
    let mut deque: LinkedDeque<i32> = Deque::new();

    assert!(deque.is_empty());
    assert_eq!(deque.remove_first(), None);
    assert_eq!(deque.remove_last(), None);

    let result = vec![4, 0, 5, 2, 3];
    let mut rit = result.iter();
    // -1 remove last
    // -2 remove first
    // extra 2 more -1 -2 will result None
    for s in vec![4, 2, 3, 0, -1, -2, 5, -2, -1, -1, -2] {
        if s == -2 {
            assert_eq!(deque.remove_first(), rit.next().map(|&v| v));
        } else if s == -1 {
            assert_eq!(deque.remove_last(), rit.next().map(|&v| v));
        } else {
            deque.add_first(s);
        }
    }

    assert!(deque.is_empty());
}


#[test]
fn test_linked_deque_size() {
    let mut deque: LinkedDeque<i32> = Deque::new();

    assert!(deque.is_empty());

    let result = vec![0, 1, 2, 3, 4, 3, 2, 3, 2, 1, 0, 0];
    let mut rit = result.iter();
    // -1 remove last
    // -2 remove first
    for s in vec![4, 2, 3, 0, -1, -2, 5, -1, -1, -2] {
        if s == -2 {
            assert_eq!(deque.size(), *rit.next().unwrap());
            deque.remove_first();
        } else if s == -1 {
            assert_eq!(deque.size(), *rit.next().unwrap());
            deque.remove_last();
        } else {
            assert_eq!(deque.size(), *rit.next().unwrap());
            deque.add_first(s);
        }
    }

    assert!(deque.is_empty());
}

#[test]
fn test_linked_deque_iter() {
    let mut deque: LinkedDeque<i32> = Deque::new();

    assert!(deque.is_empty());
    for i in 0..10 {
        if i % 2 == 0 {
            deque.add_first(i);
        } else {
            deque.add_last(i);
        }
    }

    let mut n = 0i32;
    let it = deque.iter();
    assert_eq!(it.len(), 10);

    for _ in it {
        n += 1;
    }
    assert_eq!(n, 10);
}
