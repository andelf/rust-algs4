use std::iter;

/// LSD radix sort
pub fn lsd_string_sort(a: &mut [&str]) {
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

pub trait LSD: Clone + Default {
    fn r() -> usize;
    fn width(&self) -> usize;
    fn item_at(&self, n: usize) -> usize;

    fn sort(a: &mut [Self]) {
        let w = a[0].width();
        // assume all width equals
        let n = a.len();
        let r = Self::r();

        let mut aux = iter::repeat(Default::default()).take(n).collect::<Vec<Self>>();

        // NOTE: no .rev() here
        for d in 0 .. w {
            // sort by ey-indexed counting on dth character

            // frequency counts
            let mut count = iter::repeat(0).take(r+1).collect::<Vec<usize>>();
            for i in 0 .. n {
                count[a[i].item_at(d) + 1] += 1;
            }

            // cumulates
            for r in 0 .. r {
                count[r+1] += count[r];
            }

            // move data
            for i in 0 .. n {
                aux[count[a[i].item_at(d)]] = a[i].clone();
                count[a[i].item_at(d)] += 1;
            }

            // copy back
            for i in 0 .. n {
                a[i] = aux[i].clone();
            }
        }
    }
}

impl LSD for u32 {
    fn r() -> usize { 256 }
    fn width(&self) -> usize { 4 }
    fn item_at(&self, n: usize) -> usize {
        // 0:   lowest byte
        // n-1: highest byte
        ((self >> (8 * n)) & 0xFF) as usize
    }
}

impl LSD for i16 {
    fn r() -> usize { 256 }
    fn width(&self) -> usize { 2 }
    fn item_at(&self, n: usize) -> usize {
        // 0:   lowest byte
        // n-1: highest byte
        let mut ret = (self >> (8 * n)) & 0xFF;
        // handle highest byte negative
        if n == 1 {
            if *self < 0 {
                ret -= 128;
            } else {
                ret += 128;
            }
        }
        ret as usize
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
fn test_lsd_string_sort() {
    let mut to_be_sorted = vec![
        "dab", "add", "cab", "fad", "fee", "bad",
        "bee", "fed", "bed", "ebb", "ace", "dad"];
    assert!(!is_sorted(&to_be_sorted));
    lsd_string_sort(&mut to_be_sorted);
    assert!(is_sorted(&to_be_sorted));
}


#[test]
fn test_lsd_ints_sort() {
    let mut to_be_sorted: Vec<u32> = vec![
        2830665856, 3165123597, 780293973, 3412877992, 3781910346,
        2837710927, 560466804, 2778567425, 265098057, 18593204];

    assert!(!is_sorted(&to_be_sorted));
    LSD::sort(&mut to_be_sorted);
    assert!(is_sorted(&to_be_sorted));

    let mut to_be_sorted: Vec<i16> = vec![
        19195, -5970, 2060, 2553, 26078, 15152,
        2318, 9647, -7906, -8452, -127, 128, 127];

    assert!(!is_sorted(&to_be_sorted));
    LSD::sort(&mut to_be_sorted);
    assert!(is_sorted(&to_be_sorted));

}
