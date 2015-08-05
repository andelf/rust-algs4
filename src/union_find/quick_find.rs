use std::fmt;
use super::UF;

pub struct UnionFind {
    id: Vec<usize>
}

impl UF for UnionFind {
    // Integer array id[] of length N.
    // Interpretation: p and q are connected iff they have the same id.
    fn new(n: usize) -> UnionFind {
        UnionFind { id: (0..n).collect() }
    }

    // To merge components containing p and q, change all entries
    // whose id equals id[p] to id[q].
    fn union(&mut self, p: usize, q: usize) {
        let pid = self.id[p];
        let qid = self.id[q];

        for val in self.id.iter_mut() {
            if *val == pid {
                *val = qid
            }
        }
    }

    // Check if p and q have the same id.
    fn connected(&self, p: usize, q: usize) -> bool {
        self.id[p] == self.id[q]
    }
}

impl fmt::Display for UnionFind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in self.id.iter() {
            try!(write!(f, "{} ", i));
        }
        Ok(())
    }
}
