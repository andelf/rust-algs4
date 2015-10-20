use std::iter;
use std::fmt;
use super::{QueueOfStrings, Queue};

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
        self.q[self.tail % cap] = Some(item);
        self.tail = (self.tail + 1) % cap;
        if self.q[self.tail % cap].is_some() {
            cap = 2 * cap;
            self.resize(cap);
        }
    }

    fn dequeue(&mut self) -> String {
        let cap = self.q.len();
        let item = self.q[self.head % cap].take();
        self.head = (self.head + 1) % cap;
        item.unwrap()
    }

    fn size(&self) -> usize {
        let cap = self.q.len();
        let tail = if self.tail > self.head {
            self.tail
        } else {
            self.tail + cap
        };
        tail - self.head
    }
}

// generic ResizingArrayQueue
pub struct ResizingArrayQueue<T> {
    q: Vec<Option<T>>,
    head: usize,
    tail: usize
}

impl<T: Clone> Clone for ResizingArrayQueue<T> {
    fn clone(&self) -> Self {
        ResizingArrayQueue {
            q: self.q.clone(),
            head: self.head,
            tail: self.tail
        }
    }
}

impl<T> ResizingArrayQueue<T> {
    pub fn with_capacity(capacity: usize) -> ResizingArrayQueue<T> {
        let mut storage = Vec::with_capacity(capacity);
        for _ in 0 .. capacity {
            storage.push(None);
        }

        ResizingArrayQueue {
            q: storage,
            head: 0,
            tail: 0
        }
    }

    fn resize(&mut self, capacity: usize) {
        let cap = self.q.len();
        let mut new_storage: Vec<Option<T>> = Vec::with_capacity(capacity);

        let tail = if self.tail > self.head {
            self.tail
        } else {
            self.tail + cap
        };
        for i in 0 .. capacity {
            if i >= self.head && i < tail {
                new_storage.push(self.q[i % cap].take());
            } else {
                new_storage.push(None);
            }

        }
        self.q = new_storage;
        self.tail = tail
    }
}

impl<T> Queue<T> for ResizingArrayQueue<T> {
    fn new() -> ResizingArrayQueue<T> {
        ResizingArrayQueue::with_capacity(INITIAL_QUEUE_CAPACITY)
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    fn enqueue(&mut self, item: T) {
        let mut cap = self.q.len();
        self.q[self.tail % cap] = Some(item);
        self.tail = (self.tail + 1) % cap;
        // if resize before enqueue, is_empty() will fail.
        if self.q[self.tail % cap].is_some() {
            cap = 2 * cap;
            self.resize(cap);
        }
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let cap = self.q.len();
        let item = self.q[self.head % cap].take();
        self.head = (self.head + 1) % cap;
        item
    }

    fn size(&self) -> usize {
        let cap = self.q.len();
        let tail = if self.tail > self.head {
            self.tail
        } else {
            self.tail + cap
        };
        tail - self.head
    }
}

impl<T: fmt::Debug> fmt::Debug for ResizingArrayQueue<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.head < self.tail {
            for item in self.q[self.head .. self.tail].iter() {
                try!(write!(f, "{:?}, ", item.as_ref().unwrap()));
            }
        } else {
            for item in self.q[self.head ..].iter() {
                try!(write!(f, "{:?}, ", item));
            }
            for item in self.q[.. self.tail].iter() {
                try!(write!(f, "{:?}, ", item));
            }
        }
        Ok(())
    }
}

pub struct IntoIter<T> {
    queue: ResizingArrayQueue<T>
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.queue.dequeue()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.queue.size(), Some(self.queue.size()))
    }
}

impl<T> IntoIterator for ResizingArrayQueue<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter { queue: self }
    }
}

#[test]
fn test_resizing_array_queue_of_strings() {
    let mut queue: ResizingArrayQueueOfStrings = QueueOfStrings::new();

    assert!(queue.is_empty());
    let mut result = "to be or not to be".split(' ');

    for s in "to be or not to - be - - that - - - is".split(' ') {
        if s == "-" {
            assert_eq!(&queue.dequeue(), result.next().unwrap())
        } else {
            queue.enqueue(s.into())
        }
    }
    assert!(!queue.is_empty());
    assert_eq!(2, queue.size());
}

#[test]
fn test_resizing_array_queue() {
    let mut queue: ResizingArrayQueue<String> = Queue::new();

    assert!(queue.is_empty());
    let mut result = "to be or not to be".split(' ');

    for s in "to be or not to - be - - that - - - is".split(' ') {
        if s == "-" {
            assert_eq!(queue.dequeue(), result.next().map(|s| s.into()))
        } else {
            queue.enqueue(s.into())
        }
    }
    assert!(!queue.is_empty());
    assert_eq!(2, queue.size());
}
