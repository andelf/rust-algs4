pub fn selection_sort<T: PartialOrd>(a: &mut [T]) {
    let n = a.len();
    for i in 0 .. n {
        let mut min = i;
        for j in i+1 .. n {
            if a[j] < a[min] {
                min = j;
            }
        }
        a.swap(i, min);
    }
}

pub fn insertion_sort<T: PartialOrd>(a: &mut [T]) {
    let n = a.len();
    for i in 0 .. n {
        for j in (0 .. i).rev() {
            if a[j + 1] < a[j] {
                a.swap(j+1, j);
            } else {
                break;
            }
        }
    }
}

pub fn shell_sort<T: PartialOrd>(a: &mut [T]) {
    let n  = a.len();
    let mut h = 1;

    // 3x+1 increment sequence
    while h < n / 3 {
        h = 3 * h + 1;
    }

    while h >= 1 {
        for i in h .. n {
            let mut j = i;
            loop {
                if j >= h && a[j] < a[j-h] {
                    a.swap(j, j-h);
                    j -= h;
                } else {
                    break;
                }
            }
        }
        h = h/3;
    }
}

mod mergesort;
pub use ::self::mergesort::{merge_sort, merge_bu_sort};

pub mod comparator;


pub mod quicksort;
pub use ::self::quicksort::quick_sort;

pub mod priority_queues;

mod heapsort;
pub use ::self::heapsort::heap_sort;
