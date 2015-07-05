use std::cmp;
use super::elementary_sorts::insertion_sort;

#[allow(dead_code)]
fn is_sorted<T: PartialOrd>(a: &[T]) -> bool {
    for i in 1 .. a.len() {
        if a[i] < a[i-1] {
            return false;
        }
    }
    true
}

fn merge<T: PartialOrd + Copy>(a: &mut [T], aux: &mut [T], lo: usize, mid: usize, hi: usize) {
    // assert!(is_sorted(&a[lo .. mid+1]));
    // assert!(is_sorted(&a[mid+1 .. hi+1]));

    for k in lo .. hi + 1 {
        aux[k] = a[k];
    }

    let (mut i, mut j) = (lo, mid+1);
    for k in lo .. hi + 1 {
        if i > mid {
            a[k] = aux[j];
            j += 1;
        } else if j > hi {
            a[k] = aux[i];
            i += 1;
        } else if aux[j] < aux[i] {
            a[k] = aux[j];
            j += 1;
        } else {
            a[k] = aux[i];
            i += 1;
        }
    }

    // assert!(is_sorted(&a[lo .. hi+1]));
}

const CUTOFF: usize = 7;

fn sort<T: PartialOrd + Copy>(a: &mut [T], aux: &mut [T], lo: usize, hi: usize) {
    // # original:
    // if hi <= lo  {
    //     return;
    // }
    // # Use insertion sort for small subarrays.
    if hi <= lo + CUTOFF - 1 {
        insertion_sort(&mut a[lo .. hi+1]);
        return;
    }
    // # end
    let mid = lo + (hi - lo) / 2;
    sort(a, aux, lo, mid);
    sort(a, aux, mid+1, hi);
    // Stop if already sorted.
    if !(a[mid+1] < a[mid]) {
        return;
    }
    merge(a, aux, lo, mid, hi);
}

pub fn merge_sort<T: PartialOrd + Copy>(a: &mut [T]) {
    let n = a.len();
    let mut aux: Vec<T> = a.iter().map(|&v| v).collect();
    sort(a, &mut aux, 0, n - 1);
}


/// Bottom-up mergesort
pub fn merge_bu_sort<T: PartialOrd + Copy + ::std::fmt::Debug>(a: &mut [T]) {
    let n = a.len();
    let mut aux: Vec<T> = a.iter().map(|&v| v).collect();
    let mut sz = 1;
    loop {
        if !(sz < n) { break }
        let mut lo = 0;
        loop {
            if !(lo < n - sz) { break }
            merge(a, &mut aux, lo, lo+sz-1, cmp::min(lo+sz+sz-1, n-1));
            lo += sz + sz;
        }
        sz = sz + sz;
    }
}

pub mod comparator;

#[test]
fn test_merge_sort() {
    use rand::{thread_rng, Rng};

    let mut array = thread_rng().gen_iter().take(20).collect::<Vec<u32>>();
    merge_sort(&mut array);
    assert!(is_sorted(&array));
}

#[test]
fn test_merge_bu_sort() {
    use rand::{thread_rng, Rng};

    let mut array = thread_rng().gen_iter().take(20).collect::<Vec<u32>>();
    merge_bu_sort(&mut array);
    assert!(is_sorted(&array));
}
