// use rand::{thread_rng, Rng};
use std::mem;

use super::insertion_sort;

/// quicksort partitioning
fn partition<T: PartialOrd>(a: &mut [T], lo: usize, hi: usize) -> usize {
    let mut i = lo;
    let mut j = hi + 1;

    loop {
        loop {
            i += 1;
            if a[i] < a[lo] {
                if i == hi { break }
            } else {
                break
            }
        }

        loop {
            j -= 1;
            if a[lo] < a[j] {
                if j == lo { break }
            } else {
                break
            }
        }

        if i >= j {
            break;
        }

        a.swap(i, j);
    }

    a.swap(lo, j);
    j
}


/// find median of 3, index
#[allow(dead_code)]
#[inline]
fn median_of_3<T: PartialOrd>(a: &[T], i: usize, j: usize, k: usize) -> usize {
    if a[i] >= a[j] {
        if a[j] >= a[k] {
            j
        } else {
            if a[i] >= a[k] {
                k
            } else {
                i
            }
        }
    } else {
        if a[j] >= a[k] {
            if a[i] >= a[k] {
                i
            } else {
                k
            }
        } else {
            j
        }
    }
}


// Cutoff to insertion sort for ≈ 10 items.
const CUTOFF: usize = 10;

/// quicksort optimised
fn sort<T: PartialOrd>(a: &mut [T], lo: usize, hi: usize) {
    // # small subarrays improve:
    if hi <= lo + CUTOFF - 1 {
        insertion_sort(&mut a[lo .. hi+1]);
        return ;
    }

    // # awaste of time under big arrays:
    // let m = median_of_3(a, lo, lo + (hi - lo)/2, hi);
    // a.swap(lo, m);

    let j = partition(a, lo, hi);
    // BUG FIXED: (in original code) if j == 0, j - 1 overflows
    if j > 1 {
        sort(a, lo, j-1);
    }
    sort(a, j+1, hi);
}

/// quicksort optimised
pub fn quick_sort<T: PartialOrd>(a: &mut [T]) {
    let n = a.len();
    // # time waste
    // let mut rng = thread_rng();
    // rng.shuffle(a);
    if n > 1 {
        sort(a, 0, n-1)
    }
}

/// quick-select
pub fn quick_select<T: PartialOrd>(a: &mut [T], k: usize) -> T {
    // skip StdRandom.shuffle(a);
    let mut lo = 0;
    let mut hi = a.len() - 1;
    while hi > lo {
        let j = partition(a, lo, hi);
        if j < k {
            lo = j + 1;
        } else if j > k {
            hi = j - 1;
        } else {
            break;
        }
    }
    // take the value out
    // FIXME: better to return a &T ?
    mem::replace(&mut a[k], unsafe { mem::zeroed() })
}


// for original quick sort
fn sort_orig<T: PartialOrd>(a: &mut [T], lo: usize, hi: usize) {
    if hi <= lo { return }

    let j = partition(a, lo, hi);

    if j >= 1 {
        sort_orig(a, lo, j-1);
    }
    sort_orig(a, j+1, hi);
}

/// original quick sort
pub fn quick_sort_orig<T: PartialOrd>(a: &mut [T]) {
    let n = a.len();
    if n > 1 {
        sort_orig(a, 0, n-1)
    }
}


fn sort_3way<T: PartialOrd + Copy>(a: &mut [T], lo: usize, hi: usize) {
    if hi <= lo {
        return;
    }
    let mut lt = lo;
    let mut gt = hi;
    let mut i = lo;
    // FIXME: this needs Copy
    let v = a[lo];

    while i <= gt {
        if a[i] < v {
            a.swap(lt, i);
            lt += 1;
            i += 1;
        } else if a[i] > v {
            a.swap(i, gt);
            gt -= 1;
        } else {
            i += 1;
        }
    }
    if lt >= 1 {
        sort_3way(a, lo, lt - 1);
    }
    sort_3way(a, gt + 1, hi);
}

/// 3-way quicksort
pub fn quick_sort_3way<T: PartialOrd + Copy>(a: &mut [T]) {
    let n = a.len();
    if n > 1 {
        sort_3way(a, 0, n-1)
    }
}


#[test]
fn test_median_of_3() {
    use rand::{thread_rng, Rng};
    let array = thread_rng().gen_iter().take(3).collect::<Vec<f64>>();
    let m = median_of_3(&array, 0, 1, 2);

    assert!(array[0].min(array[1]).min(array[2]) <= array[m]);
    assert!(array[m] <= array[0].max(array[1]).max(array[2]));
}
