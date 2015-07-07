// rust way of ord
use std::cmp::Ordering;


pub trait Comparator<T> {
    fn compare(&self, v: &T, w: &T) -> Ordering;

    fn less(&self, v: &T, w: &T) -> bool {
        self.compare(v, w) == Ordering::Less
    }
}

impl<T, F> Comparator<T> for F where F: Send + Sync + Fn(&T, &T) -> Ordering {
    fn compare(&self, v: &T, w: &T) -> Ordering {
        (*self)(v, w)
    }
}


pub fn insertion_sort<T: PartialOrd, C: Comparator<T>>(a: &mut [T], comparator: C) {
    let n = a.len();
    for i in 0 .. n {
        for j in (1 .. i + 1).rev() {
            if comparator.less(&a[j], &a[j-1]) {
                a.swap(j, j-1);
            }
        }
    }
}


#[test]
fn test_insertion_sort_using_a_comparator() {
    use rand::{thread_rng, Rng};

    fn is_reverse_sorted<T: PartialOrd>(a: &[T]) -> bool {
        for i in 1 .. a.len() {
            if a[i] > a[i-1] {
                return false;
            }
        }
        true
    }

    let mut array = thread_rng().gen_iter().take(10).collect::<Vec<u32>>();
    // FIXME: due to https://github.com/rust-lang/rust/issues/24680
    // the Comparator closure's type can't be inferred!
    insertion_sort(&mut array, |v: &u32, w: &u32| w.cmp(v) );
    assert!(is_reverse_sorted(&array));

}
