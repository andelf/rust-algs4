extern crate algs4;

use algs4::union_find::quick_find;
use algs4::union_find::quick_union;
use algs4::union_find::weighted_quick_union;
use algs4::union_find::improved;

fn main() {
    let mut lines = io::BufReader::new(io::stdin()).lines();

    let n = lines.next().unwrap().unwrap().parse().unwrap();
    let mut uf = quick_find::UF::new(n);

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
