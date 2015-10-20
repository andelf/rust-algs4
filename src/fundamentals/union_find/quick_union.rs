use std::fmt;
use super::UF;

pub struct UnionFind {
    id: Vec<usize>
}

impl UnionFind {
    fn root_of(&self, p: usize) -> usize {
        let mut rid = self.id[p];
        while rid != self.id[rid] {
            rid = self.id[rid];
        }
        rid
    }
}

impl UF for UnionFind {
    // Integer array id[] of length N.
    // Interpretation: id[i] is parent of i.
    // Root of i is id[id[id[...id[i]...]]].
    fn new(n: usize) -> UnionFind {
        UnionFind { id: (0..n).collect() }
    }

    // Check if p and q have the same root.
    fn connected(&self, p: usize, q: usize) -> bool {
        self.root_of(p) == self.root_of(q)
    }

    // To merge components containing p and q,
    // set the id of p's root to the id of q's root
    fn union(&mut self, p: usize, q: usize) {
        let i = self.root_of(p);
        let j = self.root_of(q);

        self.id[i] = j;
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
