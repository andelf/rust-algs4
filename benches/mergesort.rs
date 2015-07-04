#![feature(test)]

extern crate test;
extern crate rand;

extern crate algs4;

use test::{black_box, Bencher};
use rand::{thread_rng, Rng};
use algs4::mergesort::merge_sort;

const SIZE: usize = 1000;

#[bench]
fn bench_merge_sort(b: &mut Bencher) {
    let array = thread_rng().gen_iter().take(SIZE).collect::<Vec<f64>>();
    b.iter(|| {
        let mut array = array.clone();
        merge_sort(&mut array);
    });
}
