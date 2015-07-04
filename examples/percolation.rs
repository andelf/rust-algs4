extern crate algs4;
extern crate rand;

use std::iter;

use rand::{thread_rng, Rng};

use algs4::union_find::UnionFind;
use algs4::union_find::weighted_quick_union::UF;


pub struct Percolation {
    uf: UF,
    n: usize,
    opened: Vec<bool>
}

impl Percolation {
    pub fn new(n: usize) -> Percolation {
        Percolation {
            uf: UF::new(n * n),
            n: n,
            opened: iter::repeat(false).take(n * n).collect()
        }
    }

    #[inline]
    fn idx_of(&self, i: usize, j: usize) -> usize {
        (i - 1) * self.n + j - 1
    }

    fn neighbor_of(&self, i: usize, j: usize) -> ::std::vec::IntoIter<usize> {
        let n = self.n;
        let mut result: Vec<usize> = Vec::new();
        if i != 1 {
            result.push(self.idx_of(i - 1, j));
        }
        if i != n {
            result.push(self.idx_of(i + 1, j));
        }
        if j != 1 {
            result.push(self.idx_of(i, j - 1));
        }
        if j != n {
            result.push(self.idx_of(i, j + 1));
        }
        result.into_iter()
    }

    #[inline]
    pub fn is_open(&self, i: usize, j: usize) -> bool {
        self.opened[self.idx_of(i, j)]
    }

    pub fn is_full(&self, i: usize, j: usize) -> bool {
        let idx = self.idx_of(i, j);
        for col in 1 .. self.n + 1 {
            if self.uf.connected(self.idx_of(1, col), idx) {
                return true;
            }
        }
        false
    }

    // row i, column j
    pub fn open(&mut self, i: usize, j: usize) {
        let idx = self.idx_of(i, j);
        self.opened[idx] = true;

        for nid in self.neighbor_of(i, j) {
            if self.opened[nid] {
                self.uf.union(idx, nid);
            }
        }
    }

    pub fn percolates(&self) -> bool {
        for col in 1 .. self.n + 1 {
            if self.is_full(self.n, col) {
                return true;
            }
        }
        false
    }

    fn dump(&self) {
        for i in 0 .. self.n * self.n {
            if self.opened[i] {
                print!("　");
            } else {
                print!("█");
            }
            if (i + 1) % self.n == 0 {
                println!("|");
            }
        }
        println!("");
    }
}


fn stats(n: usize, t: usize) {
    let mut rng = thread_rng();

    let mut sum = 0.0;
    for _ in 0 .. t {
        let mut sites = Percolation::new(n);
        let mut cnt = 0usize;
        loop {
            let i = rng.gen_range(1, n + 1);
            let j = rng.gen_range(1, n + 1);

            sites.open(i, j);
            cnt += 1;
            if sites.percolates() {
                break
            }
        }
        sum += cnt as f64 / (n * n) as f64;
    }
    println!("mean = {}", sum / t as f64);
}


fn main() {
    let n = 30;
    let mut sites = Percolation::new(n);
    let mut rng = thread_rng();

    for _ in 0 .. 900 {
        let r = rng.gen_range(0, n * n);
        let i = r / n + 1;
        let j = r % n + 1;

        sites.open(i, j);
    }

    sites.dump();

    println!("percolates? => {}", sites.percolates());

    stats(30, 100);
}
