use std::mem;
use std::ptr;
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
        let mut sz = 0;
        while p.is_some() {
            p = p.unwrap().next.as_ref();
            sz += 1;
        }
        sz
    }
}

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
        true
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
        }
        // move in
        first.next = old_first;

        self.first = Some(first)
    }

    fn add_last(&mut self, item: T) {
        unimplemented!()
    }

    fn remove_first(&mut self) -> Option<T> {
        let old_first = self.first.take();
        let (item, first, _) = old_first.unwrap().into_item_and_pointers();
        self.first = first;
        if self.is_empty() {
            self.last = Rawlink::none()
        }
        Some(item)
    }

    fn remove_last(&mut self) -> Option<T> {
        let old_last = self.last.take();
        if old_last.p.is_null() {
            return None;
        }
        let last_but_one = unsafe { mem::transmute::<_, &mut Node<T>>(old_last.p) };

        let last: Node<T> = mem::replace(last_but_one, unsafe { mem::zeroed() });

        unsafe {
            (*last.prev.p).next = None;
        }

        Some(last.item)

    }
    // fn iter(&self) -> Iterator<Item=&T> {
    //     unimplemented!()
    // }

}


#[test]
fn test_linked_deque() {
    let mut deque: LinkedDeque<String> = Deque::new();

    let mut result = "to be not that or be".split(' ');

    for s in "to be or not to - be - - that - - - is".split(' ') {
        if s == "-" {
            assert_eq!(deque.remove_first(), Some(result.next().unwrap().into()))
        } else {
            deque.add_first(s.into())
        }
    }
}
