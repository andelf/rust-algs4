use std::iter;

/// LSD radix sort
pub fn lsd_sort_string(a: &mut [&str]) {
    let w = a[0].as_bytes().len();
    let n = a.len();
    let r = 256;

    let mut aux = iter::repeat("").take(n).collect::<Vec<&str>>();

    for d in (0 .. w).rev() {
        // sort by key-indexed counting on dth character

        // frequency counts
        let mut count = iter::repeat(0).take(r+1).collect::<Vec<usize>>();
        for i in 0 .. n {
            count[a[i].as_bytes()[d] as usize + 1] += 1;
        }

        // cumulates
        for r in 0 .. r {
            count[r+1] += count[r];
        }

        // move data
        for i in 0 .. n {
            aux[count[a[i].as_bytes()[d] as usize]] = a[i];
            count[a[i].as_bytes()[d] as usize] += 1;
        }

        // copy back
        for i in 0 .. n {
            a[i] = aux[i];
        }
    }
}

#[cfg(test)]
fn is_sorted<T: PartialOrd>(a: &[T]) -> bool {
    for i in 1 .. a.len() {
        if a[i] < a[i-1] {
            return false;
        }
    }
    true
}

#[test]
fn test_lsd_sort_string() {
    let mut to_be_sorted = vec![
        "dab", "add", "cab", "fad", "fee", "bad",
        "bee", "fed", "bed", "ebb", "ace", "dad"];
    assert!(!is_sorted(&to_be_sorted));
    lsd_sort_string(&mut to_be_sorted);
    assert!(is_sorted(&to_be_sorted));
}
