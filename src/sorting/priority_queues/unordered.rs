use super::MaxPQ;


const INITIAL_PRIORITY_QUEUE_CAPACITY: usize = 1024;

pub struct UnorderedMaxPQ<Key> {
    pq: Vec<Option<Key>>,
    n: usize
}

impl<Key: PartialOrd> UnorderedMaxPQ<Key> {
    pub fn with_capacity(capacity: usize) -> UnorderedMaxPQ<Key> {
        let mut pq = Vec::with_capacity(capacity);
        unsafe {
            pq.set_len(capacity);
        }
        UnorderedMaxPQ {
            pq: pq,
            n: 0
        }
    }
}

impl<Key: PartialOrd>  MaxPQ<Key> for UnorderedMaxPQ<Key> {
    /// create an empty priority queue
    fn new() -> Self {
        UnorderedMaxPQ::with_capacity(INITIAL_PRIORITY_QUEUE_CAPACITY)
    }
    /// create a priority queue with given keys
    fn from_vec(a: Vec<Key>) -> Self {
        let n = a.len();
        let mut pq: Vec<Option<Key>> = a.into_iter().map(|i| Some(i)).collect();
        pq.push(None);          // 1 extra space
        UnorderedMaxPQ {
            pq: pq,
            n: n
        }
    }
    /// insert a key into the priority queue
    fn insert(&mut self, x: Key) {
        self.pq[self.n] = Some(x);
        self.n += 1
    }
    /// return and remove the largest key
    fn del_max(&mut self) -> Option<Key> {
        if self.is_empty() {
            return None;
        }
        let mut max = 0;
        for i in 0 .. self.n {
            if self.pq[max] < self.pq[i] {
                max = i;
            }

        }
        self.pq.swap(max, self.n-1);
        self.n -= 1;
        self.pq[self.n].take()
    }
    /// is the priority queue empty?
    #[inline]
    fn is_empty(&self) -> bool {
        self.n == 0
    }
    /// return the largest key
    fn max(&self) -> Option<&Key> {
        if self.is_empty() {
            None
        } else {
            self.pq[self.n - 1].as_ref()
        }
    }
    /// number of entries in the priority queue
    fn size(&self) -> usize {
        self.n
    }
}


#[test]
fn test_unordered_priority_queue() {
    let mut pq: UnorderedMaxPQ<char> = MaxPQ::new();

    pq.insert('P');
    pq.insert('Q');
    pq.insert('E');

    assert_eq!(pq.size(), 3);
    assert_eq!(pq.del_max().unwrap(), 'Q');
    assert_eq!(pq.size(), 2);

    pq.insert('X');
    pq.insert('A');
    pq.insert('M');

    assert_eq!(pq.del_max().unwrap(), 'X');

    pq.insert('P');
    pq.insert('L');
    pq.insert('E');

    assert_eq!(pq.del_max().unwrap(), 'P');

    assert_eq!(pq.size(), 6);
}
