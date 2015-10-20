use std::iter;
use std::fmt;
use super::UF;

pub struct UnionFind {
    id: Vec<usize>,
    /// number of objects in the tree rooted at i.
    sz: Vec<usize>
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
    fn new(n: usize) -> UnionFind {
        UnionFind {
            id: (0..n).collect(),
            sz: iter::repeat(1).take(n).collect()
        }
    }

    fn connected(&self, p: usize, q: usize) -> bool {
        self.root_of(p) == self.root_of(q)
    }

    // Link root of smaller tree to root of larger tree.
    // Update the sz[] array.
    fn union(&mut self, p: usize, q: usize) {
        let i = self.root_of(p);
        let j = self.root_of(q);

        if i == j {
            return ;
        }
        if self.sz[i] < self.sz[j] {
            self.id[i] = j;
            self.sz[j] += self.sz[i];
        } else {
            self.id[j] = i;
            self.sz[i] += self.sz[j];
        }
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
