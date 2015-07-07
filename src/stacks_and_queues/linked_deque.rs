use std::mem;
use std::ptr;
use std::fmt;
use super::Deque;

// copy semantices
//#[derive(Copy, Clone)]
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
        let mut sz = 0;
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
        } else {
            self.last = Rawlink::some(&mut first);
        }
        // move in
        first.next = old_first;

        self.first = Some(first)
    }

    fn add_last(&mut self, item: T) {
        if self.first.is_some() {
            let old_last = self.last.take();
            let last = Box::new(Node {
                item: item,
                next: None,
                prev: Rawlink::none(),
            });
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
    // fn iter(&self) -> Iterator<Item=&T> {
    //     unimplemented!()
    // }

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



#[test]
fn test_linked_deque() {
    let mut deque: LinkedDeque<i32> = Deque::new();

    assert!(deque.is_empty());
    assert_eq!(deque.remove_first(), None);
    assert_eq!(deque.remove_last(), None);

    let result = vec![4, 0, 3, 2];
    let mut rit = result.iter();
    // -1 remove last
    // -2 remove first
    // extra 2 more -1 -2 will result None
    for s in vec![4, 2, 3, 0, -1, -2, -2, -1, -1, -2] {
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
