use std::iter;

const CUTOFF: usize = 15;

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


pub trait RadixSort: Clone + Default + PartialOrd + ::std::fmt::Debug {
    fn r() -> usize;
    fn width(&self) -> usize;
    fn item_at(&self, n: usize) -> usize;

    // sort algorithms
    fn lsd_sort(a: &mut [Self]) {
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

    fn msd_insertion(a: &mut [Self], lo: usize, hi: usize, _d: usize) {
        for i in lo .. hi+1 {
            for j in (0 .. i+1).rev() {
                // if !(j > lo && a[j].item_at(d) < a[j-1].item_at(d)) {
                if !(j > lo && a[j] < a[j-1]) {
                    break;
                }
                a.swap(j, j-1);
            }
        }
    }

    fn msd_sort_helper(a: &mut [Self], lo: usize, hi: usize, d: usize, aux: &mut [Self]) {
        if hi <= lo + CUTOFF {
            Self::msd_insertion(a, lo, hi, d);
            return;
        }

        let r = Self::r();

        // compute frequency counts (need R = 256)
        let mut count = iter::repeat(0).take(r+1).collect::<Vec<usize>>();
        for i in lo .. hi+1 {
            count[a[i].item_at(d) + 1] += 1;
        }

        // transform counts to indicies
        for r in 0 .. r {
            count[r+1] += count[r];
        }

        // distribute
        for i in lo .. hi+1 {
            aux[count[a[i].item_at(d)]] = a[i].clone();
            count[a[i].item_at(d)] += 1;
        }

        // copy back
        for i in lo .. hi+1 {
            a[i] = aux[i-lo].clone();
        }

        // no more bits
        if d == 0 {
            return ;
        }

        if count[0] > 1 {
            Self::msd_sort_helper(a, lo, lo + count[0] - 1, d-1, aux);
        }
        for r in 0 .. r {
            if count[r+1] > count[r] {
                Self::msd_sort_helper(a, lo + count[r], lo + count[r+1] - 1, d-1, aux);
            }
        }
    }

    fn msd_sort(a: &mut [Self]) {
        let n = a.len();
        let w = a[0].width();
        let mut aux = iter::repeat(Default::default()).take(n).collect::<Vec<Self>>();
        Self::msd_sort_helper(a, 0, n-1, w-1, &mut aux);
    }

}

impl RadixSort for u32 {
    fn r() -> usize { 256 }
    fn width(&self) -> usize { 4 }
    fn item_at(&self, n: usize) -> usize {
        // 0:   lowest byte
        // n-1: highest byte
        // ((self >> (8 * (4-n))) & 0xFF) as usize
        ((self >> (8 * n)) & 0xFF) as usize
    }
}

impl RadixSort for u16 {
    fn r() -> usize { 256 }
    fn width(&self) -> usize { 2 }
    fn item_at(&self, n: usize) -> usize {
        // 0:   lowest byte
        // n-1: highest byte
        ((self >> (8 * n)) & 0xFF) as usize
    }
}

impl RadixSort for i16 {
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
    RadixSort::lsd_sort(&mut to_be_sorted);
    assert!(is_sorted(&to_be_sorted));

    let mut to_be_sorted: Vec<i16> = vec![
        19195, -5970, 2060, 2553, 26078, 15152,
        2318, 9647, -7906, -8452, -127, 128, 127];

    assert!(!is_sorted(&to_be_sorted));
    RadixSort::lsd_sort(&mut to_be_sorted);
    assert!(is_sorted(&to_be_sorted));
}


#[test]
fn test_msd_ints_sort() {
    extern crate rand;
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    for sz in vec![6, 20, 1000] {
        let mut array = rng.gen_iter().take(sz).collect::<Vec<i16>>();
        assert!(!is_sorted(&array));
        RadixSort::msd_sort(&mut array);
        assert!(is_sorted(&array));

        let mut array = rng.gen_iter().take(sz).collect::<Vec<u32>>();
        assert!(!is_sorted(&array));
        RadixSort::msd_sort(&mut array);
        assert!(is_sorted(&array));
    }
}

/// 3-way string quicksort
pub fn quick_sort_3way_string(a: &mut [String]) {
    fn char_at(s: &str, d: usize) -> isize {
        assert!(d <= s.len());
        if d == s.len() {
            -1
        } else {
            s.char_at(d) as isize
        }
    }

    fn insertion(a: &mut [String], lo: usize, hi: usize, d: usize) {
        for i in lo .. hi+1 {
            for j in (0 .. i+1).rev() {
                if !(j > lo && a[j][d..] < a[j-1][d..]) {
                    break;
                }
                a.swap(j, j-1);
            }
        }
    }

    fn sort(a: &mut [String], lo: usize, hi: usize, d: usize) {
        if hi <= lo + CUTOFF {
            insertion(a, lo, hi, d);
            return;
        }

        let mut lt = lo;
        let mut gt = hi;
        let v = char_at(&a[lo], d);
        let mut i = lo + 1;
        while i <= gt {
            let t = char_at(&a[i], d);
            if t < v {
                a.swap(lt, i);
                lt += 1;
                i += 1;
            } else if t > v {
                a.swap(i, gt);
                gt -= 1;
            } else {
                i += 1;
            }
        }

        sort(a, lo, lt-1, d);
        if v >= 0 {
            sort(a, lt, gt, d+1);
        }
        sort(a, gt+1, hi, d);
    }
    let n = a.len();
    sort(a, 0, n-1, 0);
}

#[test]
fn test_quick_sort_3way_string() {
    let mut shell = "she sells seashells by the sea shore the shells she sells are surely seashells".split(' ').map(|s| s.into()).collect::<Vec<String>>();
    assert!(!is_sorted(&shell));
    quick_sort_3way_string(&mut shell);
    assert!(is_sorted(&shell));

}
