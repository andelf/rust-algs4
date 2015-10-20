#![feature(test)]

extern crate test;
extern crate rand;

extern crate algs4;

use rand::{thread_rng, Rng};
use algs4::sorting::*;
use algs4::sorting::quicksort::{quick_sort_3way, quick_sort_orig, quick_select};


fn is_sorted<T: PartialOrd>(a: &[T]) -> bool {
    for i in 1 .. a.len() {
        if a[i] < a[i-1] {
            return false;
        }
    }
    true
}


macro_rules! deftest(
    ($name:ident, $func:ident) => (
        deftest!($name, $func, f64);
    );
    ($name:ident, $func:ident, $typ:ty) => (
        #[test]
        fn $name() {
            let mut rng = thread_rng();
            for sz in vec![0, 1, 2, 3, 10, 20, 1000] {
                let mut array = rng.gen_iter().take(sz).collect::<Vec<$typ>>();
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
deftest!(test_quick_sort_orig, quick_sort_orig);
deftest!(test_quick_sort_3way, quick_sort_3way);

deftest!(test_heap_sort, heap_sort);

// # Special Sorting
deftest!(test_quick_sort_3way_on_u8_array, quick_sort_3way, u8);

// # Non-Sorting

#[test]
fn test_quick_select() {
    let mut rng = thread_rng();

    for sz in vec![1, 2, 10, 20] {
        let array: Vec<usize> = (0 .. sz).collect();

        for i in 0 .. sz {
            let mut array = array.clone();
            rng.shuffle(&mut array);
            // the (i-1)th item is i
            assert_eq!(quick_select(&mut array, i), i);
        }
    }
}
