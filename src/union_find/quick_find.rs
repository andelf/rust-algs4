use std::io::prelude::*;
use std::io;

pub struct UF {
    n: usize,
    id: Vec<usize>
}

impl UF {
    // Integer array id[] of length N.
    // Interpretation: p and q are connected iff they have the same id.
    pub fn new(n: usize) -> UF {
        UF { n: n, id: (0..n).collect() }
    }

    // To merge components containing p and q, change all entries
    // whose id equals id[p] to id[q].
    pub fn union(&mut self, p: usize, q: usize) {
        let pid = self.id[p];
        let qid = self.id[q];

        for val in self.id.iter_mut() {
            if *val == pid {
                *val = qid
            }
        }
    }

    // Check if p and q have the same id.
    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.id[p] == self.id[q]
    }

    pub fn find(&self, p: usize) -> usize {
        unimplemented!()
    }

    pub fn count(&self) -> usize {
        unimplemented!()
    }

    fn dump(&self) {
        for i in self.id.iter() {
            print!("{} ", i);
        }
        println!("")
    }
}


fn main() {
    let mut lines = io::BufReader::new(io::stdin()).lines();

    let n = lines.next().unwrap().unwrap().parse().unwrap();
    let mut uf = UF::new(n);

    for line in lines {
        let segs: Vec<usize> = line.unwrap().split(' ').map(|n| n.parse().unwrap()).collect();
        let p = segs[0];
        let q = segs[1];

        if !uf.connected(p, q) {
            uf.union(p, q);
            println!("{} {}", p, q)
        }
    }

    uf.dump()
}
