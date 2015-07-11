use super::MaxPQ;


const INITIAL_PRIORITY_QUEUE_CAPACITY: usize = 512;

pub struct BinaryHeapMaxPQ<Key> {
    pq: Vec<Option<Key>>,
    n: usize
}


impl<Key: PartialOrd> BinaryHeapMaxPQ<Key> {
    pub fn with_capacity(capacity: usize) -> BinaryHeapMaxPQ<Key> {
        let mut pq = Vec::with_capacity(capacity + 1);
        unsafe {
            pq.set_len(capacity + 1);
        }
        BinaryHeapMaxPQ {
            pq: pq,
            n: 0
        }
    }

    fn swim(&mut self, k: usize) {
        let mut k = k;
        while k > 1 && self.pq[k/2] < self.pq[k] {
            self.pq.swap(k, k/2);
            k = k/2;
        }
    }

    fn sink(&mut self, k: usize) {
        let mut k = k;
        while 2*k <= self.n {
            let mut j = 2*k;
            if j < self.n && self.pq[j] < self.pq[j+1] {
                j += 1;
            }
            if self.pq[k] >= self.pq[j] {
                break;
            }
            self.pq.swap(k, j);
            k = j;
        }
    }
}

impl<Key: PartialOrd>  MaxPQ<Key> for BinaryHeapMaxPQ<Key> {
    /// create an empty priority queue
    fn new() -> Self {
        BinaryHeapMaxPQ::with_capacity(INITIAL_PRIORITY_QUEUE_CAPACITY)
    }
    /// create a priority queue with given keys
    fn from_vec(a: Vec<Key>) -> Self {
        let mut pq = Self::new();
        let len = a.len();
        pq.pq.move_from(a.map_in_place(Some), 0, len);
        pq
    }
    /// insert a key into the priority queue
    fn insert(&mut self, x: Key) {
        self.n += 1;
        let n = self.n;
        self.pq[n] = Some(x);
        self.swim(n);
    }
    /// return and remove the largest key
    fn del_max(&mut self) -> Option<Key> {
        let max = self.pq[1].take();
        self.pq.swap(1, self.n);
        self.n -= 1;
        self.sink(1);
        max
    }
    /// is the priority queue empty?
    #[inline]
    fn is_empty(&self) -> bool {
        self.n == 0
    }
    /// return the largest key
    fn max(&self) -> Option<&Key> {
        self.pq[1].as_ref()
    }
    /// number of entries in the priority queue
    fn size(&self) -> usize {
        self.n
    }
}


#[test]
fn test_binary_priority_queue() {
    let mut pq: BinaryHeapMaxPQ<char> = MaxPQ::new();

    pq.insert('P');
    pq.insert('Q');
    pq.insert('E');

    assert_eq!(pq.size(), 3);
    println!("==> {:?}", pq.pq);
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

#[test]
fn test_empty() {
    let mut pq: BinaryHeapMaxPQ<char> = MaxPQ::new();
    pq.insert('P');
    assert_eq!(pq.is_empty(), false);
    assert_eq!(pq.size(), 1);
    pq.del_max();
    assert_eq!(pq.size(), 0);
    assert_eq!(pq.is_empty(), true);
}
