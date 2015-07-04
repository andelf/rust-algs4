use std::mem;
use std::ptr;
use super::QueueOfStrings;

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

struct Node {
    item: String,
    next: Option<Box<Node>>,

}

impl Node {
    /// work around for moved value
    fn into_item_and_next(self) -> (String, Option<Box<Node>>) {
        (self.item, self.next)
    }
}

pub struct LinkedQueueOfStrings {
    first: Option<Box<Node>>,
    last: Rawlink<Node>
}

impl QueueOfStrings for LinkedQueueOfStrings {
    fn new() -> LinkedQueueOfStrings {
        LinkedQueueOfStrings { first: None, last: Rawlink::none() }
    }

    fn is_empty(&self) -> bool {
        self.first.is_none()
    }

    fn enqueue(&mut self, item: String) {
        let ref old_last = self.last.take();
        let mut last = Box::new(Node {
            item: item,
            next: None
        });
        self.last = Rawlink::some(&mut last);
        if self.is_empty() {
            self.first = Some(last)
        } else {
            unsafe {
                (*old_last.p).next = Some(last)
            }
        }
    }

    fn dequeue(&mut self) -> String {
        let old_first = self.first.take();
        let (item, first) = old_first.unwrap().into_item_and_next();
        self.first = first;
        if self.is_empty() {
            self.last = Rawlink::none()
        }
        item
    }
}


#[test]
fn test_linked_queue() {
    let mut queue: LinkedQueueOfStrings = QueueOfStrings::new();

    let mut result = "to be or not to be".split(' ');

    for s in "to be or not to - be - - that - - - is".split(' ') {
        if s == "-" {
            assert_eq!(&queue.dequeue(), result.next().unwrap())
        } else {
            queue.enqueue(s.into())
        }
    }
}
