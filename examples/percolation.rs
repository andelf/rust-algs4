#![feature(test)]
extern crate algs4;
extern crate rand;
extern crate test;

use std::env;
use std::iter;

use test::stats::Stats;
use rand::{thread_rng, Rng};

use algs4::union_find::UF;
use algs4::union_find::weighted_quick_union::UnionFind;

/// a percolation system using an N-by-N grid of sites
pub struct Percolation {
    uf: UnionFind,
    n: usize,
    opened: Vec<bool>
}

impl Percolation {
    /// create N-by-N grid, with all sites blocked
    pub fn new(n: usize) -> Percolation {
        Percolation {
            uf: UnionFind::new(n * n),
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

    /// is site (row i, column j) open?
    #[inline]
    pub fn is_open(&self, i: usize, j: usize) -> bool {
        self.opened[self.idx_of(i, j)]
    }

    /// is site (row i, column j) full?
    pub fn is_full(&self, i: usize, j: usize) -> bool {
        let idx = self.idx_of(i, j);
        for col in 1 .. self.n + 1 {
            if self.is_open(1, col) && self.uf.connected(self.idx_of(1, col), idx) {
                return true;
            }
        }
        false
    }

    /// open site (row i, column j) if it is not open already
    pub fn open(&mut self, i: usize, j: usize) {
        assert!(i >= 1 && i <= self.n && j >= 1 && j <= self.n, "(i, j) are out of bounds");
        let idx = self.idx_of(i, j);
        self.opened[idx] = true;

        for nid in self.neighbor_of(i, j) {
            if self.opened[nid] {
                self.uf.union(idx, nid);
            }
        }
    }

    /// does the system percolate?
    pub fn percolates(&self) -> bool {
        for col in 1 .. self.n + 1 {
            if self.is_open(self.n, col) && self.is_full(self.n, col) {
                return true;
            }
        }
        false
    }

    #[allow(dead_code)]
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

pub struct PercolationStats {
    t: usize,
    xs: Vec<f64>
}

impl PercolationStats {
    /// perform T independent experiments on an N-by-N grid
    pub fn new(n: usize, t: usize) -> PercolationStats {
        let mut rng = thread_rng();
        let mut xs = Vec::new();
        let n_sites = (n as f64) * (n as f64);
        for _ in 0 .. t {
            let mut sites = Percolation::new(n);
            let mut cnt = 0f64;
            loop {
                let r = rng.gen_range(0, n * n);
                let i = r / n + 1;
                let j = r % n + 1;

                // only open new site
                if sites.is_open(i, j) {
                    continue;
                }
                sites.open(i, j);
                cnt += 1.0;
                if sites.percolates() {
                    break
                }
            }
            xs.push(cnt / n_sites);
        }
        PercolationStats { t: t, xs: xs }
    }

    /// sample mean of percolation threshold
    pub fn mean(&self) -> f64 {
        self.xs.mean()
    }

    /// sample standard deviation of percolation threshold
    pub fn std_dev(&self) -> f64 {
        self.xs.std_dev()
    }

    /// low  endpoint of 95% confidence interval
    pub fn confidence_low(&self) -> f64 {
        self.xs.mean() - 1.96 * self.std_dev() / (self.t as f64).sqrt()
    }

    /// high endpoint of 95% confidence interval
    pub fn confidence_high(&self) -> f64 {
        self.xs.mean() + 1.96 * self.std_dev() / (self.t as f64).sqrt()
    }
}


fn main() {

    let mut args = env::args();

    args.next();
    let n: usize = args.next().unwrap().parse().unwrap();
    let t: usize = args.next().unwrap().parse().unwrap();

    if t < 30 {
        println!("Warning: T is not sufficiently large! (at least 30).");
    }

    let stats = PercolationStats::new(n, t);
    println!("{:23} = {}", "mean", stats.mean());
    println!("{:23} = {}", "stddev", stats.std_dev());
    println!("{:23} = {}, {}", "95% confidence interval", stats.confidence_low(), stats.confidence_high());
}
