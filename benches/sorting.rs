#![feature(test)]

extern crate test;
extern crate rand;

extern crate algs4;

use test::Bencher;
use rand::{thread_rng, Rng};
use algs4::elementary_sorts::*;
use algs4::mergesort::*;
use algs4::quicksort::*;


static SIZE: usize = 1000;
// for small array
// static SIZE: usize = 10;

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
defbench!(bench_insertion_sort, insertion_sort);
defbench!(bench_shell_sort, shell_sort);

defbench!(bench_merge_sort, merge_sort);
defbench!(bench_merge_bu_sort, merge_bu_sort);

defbench!(bench_quick_sort, quick_sort);


#[bench]
fn bench_knuth_shuffle(b: &mut Bencher) {
    let array = thread_rng().gen_iter().take(SIZE).collect::<Vec<f64>>();
    b.iter(|| {
        let mut array = array.clone();
        knuth_shuffle(&mut array);
    });
}
