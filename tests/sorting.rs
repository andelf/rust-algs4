#![feature(test)]

extern crate test;
extern crate rand;

extern crate algs4;

use rand::{thread_rng, Rng};
use algs4::elementary_sorts::*;
use algs4::mergesort::*;
use algs4::quicksort::*;


fn is_sorted<T: PartialOrd>(a: &[T]) -> bool {
    for i in 1 .. a.len() {
        if a[i] <= a[i-1] {
            return false;
        }
    }
    true
}


macro_rules! deftest(
    ($name:ident, $func:ident) => (
        #[test]
        fn $name() {
            let mut rng = thread_rng();
            for sz in vec![0, 1, 2, 3, 10, 20, 1000] {
                let mut array = rng.gen_iter().take(sz).collect::<Vec<f64>>();
                $func(&mut array);
                assert!(is_sorted(&array));
            }
        }
    )
);


deftest!(test_selection_sort, selection_sort);
deftest!(test_insertion_sort, insertion_sort);
deftest!(test_shell_sort, shell_sort);

deftest!(test_merge_sort, merge_sort);
deftest!(test_merge_bu_sort, merge_bu_sort);

deftest!(test_quick_sort, quick_sort);
