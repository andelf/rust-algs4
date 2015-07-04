use std::iter;
use super::QueueOfStrings;

const INITIAL_QUEUE_CAPACITY: usize = 2;

pub struct ResizingArrayQueueOfStrings {
    q: Vec<Option<String>>,
    head: usize,
    tail: usize

}

impl ResizingArrayQueueOfStrings {
    pub fn with_capacity(capacity: usize) -> ResizingArrayQueueOfStrings {
        let storage = iter::repeat(None).take(capacity).collect();

        ResizingArrayQueueOfStrings {
            q: storage,
            head: 0,
            tail: 0
        }
    }

    fn resize(&mut self, capacity: usize) {
        let cap = self.q.len();
        let mut new_storage: Vec<Option<String>> = iter::repeat(None).take(capacity).collect();
        let tail = if self.tail > self.head {
            self.tail
        } else {
            self.tail + cap
        };
        for i in self.head .. tail{
            new_storage[i] = self.q[i % cap].take();
        }
        self.q = new_storage;
        // self.head = self.head
        self.tail = tail
    }
}

impl QueueOfStrings for ResizingArrayQueueOfStrings {
    fn new() -> ResizingArrayQueueOfStrings {
        let storage = iter::repeat(None).take(INITIAL_QUEUE_CAPACITY).collect();

        ResizingArrayQueueOfStrings {
            q: storage,
            head: 0,
            tail: 0
        }
    }

    fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    fn enqueue(&mut self, item: String) {
        let mut cap = self.q.len();
        if self.q[self.tail % cap].is_some() {
            cap = 2 * cap;
            self.resize(cap);
        }
        self.q[self.tail % cap] = Some(item);
        self.tail = (self.tail + 1) % cap
    }

    fn dequeue(&mut self) -> String {
        let cap = self.q.len();
        let item = self.q[self.head % cap].take();
        self.head = (self.head + 1) % cap;
        item.unwrap()
    }
}


#[test]
fn test_resizing_array_queue() {
    let mut queue: ResizingArrayQueueOfStrings = QueueOfStrings::new();

    let mut result = "to be or not to be".split(' ');

    for s in "to be or not to - be - - that - - - is".split(' ') {
        if s == "-" {
            assert_eq!(&queue.dequeue(), result.next().unwrap())
        } else {
            queue.enqueue(s.into())
        }
    }
}
