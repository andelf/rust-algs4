pub fn is_sorted<T: PartialOrd>(a: &[T]) -> bool {
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


fn sort<T: PartialOrd + Copy>(a: &mut [T], aux: &mut [T], lo: usize, hi: usize) {
    if hi <= lo {
        return;
    }
    let mid = lo + (hi - lo) / 2;
    sort(a, aux, lo, mid);
    sort(a, aux, mid+1, hi);
    merge(a, aux, lo, mid, hi);
}

pub fn merge_sort<T: PartialOrd + Copy>(a: &mut [T]) {
    let n = a.len();
    let mut aux: Vec<T> = a.iter().map(|&v| v).collect();
    sort(a, &mut aux[..], 0, n - 1);
}


#[test]
fn test_merge_sort() {
    use rand::{thread_rng, Rng};

    let mut array = thread_rng().gen_iter().take(10).collect::<Vec<u32>>();
    merge_sort(&mut array);
    assert!(is_sorted(&array));
}
