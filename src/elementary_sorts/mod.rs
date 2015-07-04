pub fn is_sorted<T: PartialOrd>(a: &[T]) -> bool {
    for i in 1 .. a.len() {
        if a[i] < a[i-1] {
            return false;
        }
    }
    true
}



pub fn selection_sort<T: PartialOrd>(a: &mut [T]) {
    let n = a.len();
    for i in 0 .. n {
        let mut min = i;
        for j in i+1 .. n {
            if a[j] < a[min] {
                min = j;
            }
        }
        a.swap(i, min);
    }
}


#[test]
fn test_selection_sort() {
    extern crate rand;

    let mut array = [0f64; 100];
    for i in 0 .. 100 {
        array[i] = rand::random();
    }
    selection_sort(&mut array);
    assert!(is_sorted(&array));
}
