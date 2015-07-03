use std::io::prelude::*;
use std::io;
use std::iter;

/// Quick union with path compression.
pub struct UF {
    n: usize,
    id: Vec<usize>,
    //  number of objects in the tree rooted at i.
    sz: Vec<usize>
}

impl UF {
    pub fn new(n: usize) -> UF {
        UF {
            n: n,
            id: (0..n).collect(),
            sz: iter::repeat(1).take(n).collect()
        }
    }

    fn root_of(&mut self, p: usize) -> usize {
        let mut rid = self.id[p];
        while rid != self.id[rid] {
            // Simpler one-pass variant
            self.id[rid] = self.id[self.id[rid]];
            rid = self.id[rid];
        }
        rid
    }

    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        self.root_of(p) == self.root_of(q)
    }

    // Link root of smaller tree to root of larger tree.
    //Update the sz[] array.
    pub fn union(&mut self, p: usize, q: usize) {
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


#[test]
fn it_works() {

}
