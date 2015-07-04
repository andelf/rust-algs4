#![feature(test)]

extern crate test;
extern crate rand;

extern crate algs4;

use test::{black_box, Bencher};

use algs4::union_find::UnionFind;
use algs4::union_find::quick_find;
use algs4::union_find::quick_union;
use algs4::union_find::weighted_quick_union;
use algs4::union_find::improved;

const NUM_OF_OBJECTS: usize = 1000;
const NUM_OF_OPERATIONS: usize = 50000;

#[bench]
fn bench_quick_find(b: &mut Bencher) {

    let mut paths: Vec<(usize, usize)> = Vec::new();
    for _ in 0..NUM_OF_OPERATIONS {
        paths.push((rand::random::<usize>() % NUM_OF_OBJECTS, rand::random::<usize>() % NUM_OF_OBJECTS));
    }

    b.iter(|| {
        let mut uf = quick_find::UF::new(NUM_OF_OBJECTS);
        for &(p, q) in paths.iter() {
            black_box(uf.union(p, q));
        }
    });
}



#[bench]
fn bench_quick_union(b: &mut Bencher) {
    let mut paths: Vec<(usize, usize)> = Vec::new();
    for _ in 0..NUM_OF_OPERATIONS {
        paths.push((rand::random::<usize>() % NUM_OF_OBJECTS,
                    rand::random::<usize>() % NUM_OF_OBJECTS));
    }

    b.iter(|| {
        let mut uf = quick_union::UF::new(NUM_OF_OBJECTS);
        for &(p, q) in paths.iter() {
            black_box(uf.union(p, q));
        }
    });
}


#[bench]
fn bench_weighted_quick_union(b: &mut Bencher) {

    let mut paths: Vec<(usize, usize)> = Vec::new();
    for _ in 0..NUM_OF_OPERATIONS {
        paths.push((rand::random::<usize>() % NUM_OF_OBJECTS,
                    rand::random::<usize>() % NUM_OF_OBJECTS));
    }

    b.iter(|| {
        let mut uf = weighted_quick_union::UF::new(NUM_OF_OBJECTS);
        for &(p, q) in paths.iter() {
            black_box(uf.union(p, q));
        }
    });
}

#[bench]
fn bench_quick_union_with_path_compression_one_pass(b: &mut Bencher) {

    let mut paths: Vec<(usize, usize)> = Vec::new();
    for _ in 0..NUM_OF_OPERATIONS {
        paths.push((rand::random::<usize>() % NUM_OF_OBJECTS,
                    rand::random::<usize>() % NUM_OF_OBJECTS));
    }

    b.iter(|| {
        let mut uf = improved::UF::new(NUM_OF_OBJECTS);
        for &(p, q) in paths.iter() {
            black_box(uf.union(p, q));
        }
    });
}
