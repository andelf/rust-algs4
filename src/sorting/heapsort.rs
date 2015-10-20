// 1-based index to 0-based
#[inline]
fn less<T: PartialOrd>(a: &mut [T], i: usize, j: usize) -> bool {
    a[i-1] < a[j-1]
}

#[inline]
fn sink<T: PartialOrd>(a: &mut [T], k: usize, n: usize) {
    let mut k = k;
    while 2*k <= n {
        let mut j = 2*k;

        if j < n && less(a, j, j+1) {
            j += 1;
        }
        if !less(a, k, j) {
            break;
        }
        // 0-based index
        a.swap(k-1, j-1);
        k = j;
    }
}

#[inline]
pub fn heap_sort<T: PartialOrd>(a: &mut [T]) {
    let mut n = a.len();

    for k in (1 .. n/2 + 1).rev() {
        sink(a, k, n);
    }
    while n > 1 {
        // 0-based index
        a.swap(0, n-1);
        n -= 1;
        sink(a, 1, n);
    }
}
