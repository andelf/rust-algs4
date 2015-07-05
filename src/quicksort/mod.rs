// use rand::{thread_rng, Rng};
use super::elementary_sorts::insertion_sort;

// TODO: distingush improved version vs original version


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
fn median_of_3<T: PartialOrd>(a: &[T], i: usize, j: usize, k: usize) -> usize {
    //                           lo, lo + (hi - lo) / 2, hi)
    use std::cmp::Ordering::{Greater, Less, Equal};

    let i_j = a[i].partial_cmp(&a[j]).unwrap_or(Equal);
    let j_k = a[j].partial_cmp(&a[k]).unwrap_or(Equal);
    let i_k = a[i].partial_cmp(&a[k]).unwrap_or(Equal);

    // decision tree
    match (i_j, j_k, i_k) {
        (Greater, Greater, _)    | (Less, Less, _)          => j,
        (Greater, Less, Less)    | (Less, Greater, Greater) => i,
        (Greater, Less, Greater) | (Less, Greater, Less)    => k,
        (Equal, _, _)            | (_, _, Equal)            => i,
        _                                                   => j
    }
}


// Cutoff to insertion sort for ≈ 10 items.
const CUTOFF: usize = 10;

fn sort<T: PartialOrd>(a: &mut [T], lo: usize, hi: usize) {
    // # original:
    // if hi <= lo { return }
    // # small subarrays improve:
    if hi <= lo + CUTOFF - 1 {
        insertion_sort(&mut a[lo .. hi+1]);
        return ;
    }
    // # end

    // waste of time under big arrays
    // let m = median_of_3(a, lo, lo + (hi - lo)/2, hi);
    // a.swap(lo, m);

    let j = partition(a, lo, hi);
    // BUG FIXED: (in original code) if j == 0, j - 1 overflows
    if j > 0 {
        sort(a, lo, j-1);
    }
    sort(a, j+1, hi);
}

pub fn quick_sort<T: PartialOrd>(a: &mut [T]) {
    let n = a.len();
    if n <= 1 {
        return;
    }
    // # time waste
    // let mut rng = thread_rng();
    // rng.shuffle(a);
    sort(a, 0, n-1);
}


#[test]
fn test_median_of_3() {
    use rand::{thread_rng, Rng};
    let array = thread_rng().gen_iter().take(3).collect::<Vec<f64>>();
    let m = median_of_3(&array, 0, 1, 2);

    assert!(array[0].min(array[1]).min(array[2]) <= array[m]);
    assert!(array[m] <= array[0].max(array[1]).max(array[2]));
}
