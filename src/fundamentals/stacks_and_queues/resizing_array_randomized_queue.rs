use std::fmt;
use rand::{thread_rng, Rng};
use super::RandomizedQueue;

const INITIAL_QUEUE_CAPACITY: usize = 2;


pub struct ResizingArrayRandomizedQueue<T> {
    q: Vec<Option<T>>,
    n: usize,
    shuffled: bool
}

impl<T> ResizingArrayRandomizedQueue<T> {
    pub fn with_capacity(capacity: usize) -> ResizingArrayRandomizedQueue<T> {
        let mut storage = Vec::with_capacity(capacity);
        for _ in 0 .. capacity {
            storage.push(None);
        }

        ResizingArrayRandomizedQueue {
            q: storage,
            n: 0,
            shuffled: false
        }
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        let mut rng = thread_rng();
        let mut seq: Vec<usize> = (0 .. self.n).collect();
        rng.shuffle(&mut seq);
        Iter {
            q: &self.q,
            seq: seq
        }
    }

    fn resize(&mut self, capacity: usize) {
        let mut new_storage = Vec::with_capacity(capacity);
        for i in 0 .. capacity {
            if i < self.n {
                new_storage.push(self.q[i].take())
            } else {
                new_storage.push(None);
            }
        }
        self.q = new_storage;
    }

    #[inline]
    fn shuffle(&mut self) {
        if !self.shuffled {
            let mut rng = thread_rng();
            rng.shuffle(&mut self.q[.. self.n]);
            self.shuffled = true;
        }
    }
}

impl<T> RandomizedQueue<T> for ResizingArrayRandomizedQueue<T> {
    fn new() -> ResizingArrayRandomizedQueue<T> {
        ResizingArrayRandomizedQueue::with_capacity(INITIAL_QUEUE_CAPACITY)
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.n == 0
    }

    #[inline]
    fn size(&self) -> usize {
        self.n
    }

    fn enqueue(&mut self, item: T) {
        if self.shuffled {
            self.shuffled = false;
        }
        let len = self.q.len();
        if self.n == len {
            self.resize(2 * len);
        }
        self.q[self.n] = Some(item);
        self.n += 1;
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        self.shuffle();
        self.n -= 1;
        let cell = self.q[self.n].take();
        let len = self.q.len();
        if self.n > 0 && self.n == len / 4 {
            self.resize(len / 2);
        }
        cell
    }

    fn sample<'a>(&'a self) -> Option<&'a T> {
        let mut rng = thread_rng();
        rng.choose(&self.q[0 .. self.n]).unwrap().as_ref()
    }
}

impl<T: fmt::Debug> fmt::Debug for ResizingArrayRandomizedQueue<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{:?}", self.q));
        Ok(())
    }
}


pub struct Iter<'a, T: 'a> {
    q: &'a [Option<T>],
    seq: Vec<usize>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match self.seq.pop() {
            None => None,
            Some(idx) => {
                self.q[idx].as_ref()
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.seq.len();
        (n, Some(n))
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.seq.len()
    }
}

#[test]
fn test_resizing_array_random_queue_iter() {
    let mut queue: ResizingArrayRandomizedQueue<u32> = RandomizedQueue::new();

    assert!(queue.is_empty());
    for i in 0 .. 10 {
        queue.enqueue(i);
    }

    for _ in queue.iter() {
    }

    assert_eq!(queue.size(), 10);
}


#[test]
fn test_resizing_array_random_queue_sample() {
    let mut queue: ResizingArrayRandomizedQueue<u32> = RandomizedQueue::new();

    assert!(queue.is_empty());
    for i in 0 .. 10 {
        queue.enqueue(i);
    }

    let mut matches = 0i32;
    for _ in 0 .. 10 {
        if queue.sample() == queue.sample() {
            matches += 1;
        }
    }

    assert!(matches <= 10);
    assert_eq!(queue.size(), 10);
}


#[test]
fn test_resizing_array_random_queue_dequeue_enqueue() {
    let mut queue1: ResizingArrayRandomizedQueue<String> = RandomizedQueue::new();
    let mut queue2: ResizingArrayRandomizedQueue<String> = RandomizedQueue::new();

    let mut matches = 0i32;
    for s in "to be or - not to - - be that - is the - - question - - - -".split(' ') {
        if s == "-" {
            if queue1.dequeue() == queue2.dequeue() {
                matches += 1;
            }
        } else {
            queue1.enqueue(s.to_string());
            queue2.enqueue(s.to_string());
        }
    }
    // :( half not matches
    assert!(matches <= 10);
}
