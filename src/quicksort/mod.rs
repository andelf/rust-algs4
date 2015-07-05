// use rand::{thread_rng, Rng};

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


fn sort<T: PartialOrd>(a: &mut [T], lo: usize, hi: usize) {
    if hi <= lo { return }
    let j = partition(a, lo, hi);
    // BUG in original: if j == 0, j - 1 overflows
    if j != 0 {
        sort(a, lo, j-1);
    }
    sort(a, j+1, hi);
}


pub fn quick_sort<T: PartialOrd>(a: &mut [T]) {
    let n = a.len();
    // time waste
    // let mut rng = thread_rng();
    // rng.shuffle(a);
    sort(a, 0, n-1);
}


#[test]
fn test_quick_sort() {
    use rand::{thread_rng, Rng};
    fn is_sorted<T: PartialOrd>(a: &[T]) -> bool {
        for i in 1 .. a.len() {
            if a[i] < a[i-1] {
                return false;
            }
        }
        true
    }

    let mut array = thread_rng().gen_iter().take(10).collect::<Vec<u32>>();
    quick_sort(&mut array);
    assert!(is_sorted(&array));
}
