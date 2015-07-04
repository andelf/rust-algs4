#![feature(test)]

extern crate test;
extern crate rand;

extern crate algs4;

use test::{black_box, Bencher};
use rand::{thread_rng, Rng};
use algs4::elementary_sorts::*;


static SIZE: usize = 1000;


macro_rules! defbench(
    ($name:ident, $func:ident) => (
        #[bench]
        fn $name(b: &mut Bencher) {
            let array = thread_rng().gen_iter().take(SIZE).collect::<Vec<f64>>();
            b.iter(|| {
                let mut array = array.clone();
                $func(&mut array);
            });

        }
    )
);


defbench!(bench_selection_sort, selection_sort);
