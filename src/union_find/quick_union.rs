use std::fmt;

pub struct UF {
    n: usize,
    id: Vec<usize>
}

impl UF {
    // Integer array id[] of length N.
    // Interpretation: id[i] is parent of i.
    // Root of i is id[id[id[...id[i]...]]].
    pub fn new(n: usize) -> UF {
        UF { n: n, id: (0..n).collect() }
    }

    fn root_of(&self, p: usize) -> usize {
        let mut rid = self.id[p];
        while rid != self.id[rid] {
            rid = self.id[rid];
        }
        rid
    }

    // Check if p and q have the same root.
    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.root_of(p) == self.root_of(q)
    }

    // To merge components containing p and q,
    // set the id of p's root to the id of q's root
    pub fn union(&mut self, p: usize, q: usize) {
        let i = self.root_of(p);
        let j = self.root_of(q);

        self.id[i] = j;
    }

    pub fn find(&self, p: usize) -> usize {
        unimplemented!()
    }

    pub fn count(&self) -> usize {
        unimplemented!()
    }
}

impl fmt::Display for UF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in self.id.iter() {
            write!(f, "{} ", i);
        }
        Ok(())
    }
}
