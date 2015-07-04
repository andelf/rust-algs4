use rand::{thread_rng, Rng};

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

pub fn insertion_sort<T: PartialOrd>(a: &mut [T]) {
    let n = a.len();
    for i in 0 .. n {
        for j in (0 .. i).rev() {
            if a[j + 1] < a[j] {
                a.swap(j+1, j);
            } else {
                break;
            }
        }
    }
}

pub fn shell_sort<T: PartialOrd>(a: &mut [T]) {
    let n  = a.len();

    let mut h = 1;

    // 3x+1 increment sequence
    while h < n / 3 {
        h = 3 * h + 1;
    }

    while h >= 1 {
        for i in h .. n {
            let mut j = i;
            loop {
                if j >= h && a[j] < a[j-h] {
                    a.swap(j, j-h);
                    j -= h;
                } else {
                    break;
                }
            }
        }
        h = h/3;
    }
}

pub fn knuth_shuffle<T>(a: &mut [T]) {
    let mut rng = thread_rng();

    let n = a.len();

    for i in 0 .. n {
        let r = rng.gen_range(0, i+1);
        a.swap(i, r);
    }
}

#[test]
fn test_selection_sort() {
    use rand;
    let mut array = [0f64; 100];
    for i in 0 .. 100 {
        array[i] = rand::random();
    }
    selection_sort(&mut array);
    assert!(is_sorted(&array));
}


#[test]
fn test_insertion_sort() {
    use rand;
    let mut array = [0f64; 100];
    for i in 0 .. 100 {
        array[i] = rand::random();
    }
    insertion_sort(&mut array);
    assert!(is_sorted(&array));
}

#[test]
fn test_shell_sort() {
    use rand;
    let mut array = [0f64; 100];
    for i in 0 .. 100 {
        array[i] = rand::random();
    }
    shell_sort(&mut array);
    assert!(is_sorted(&array));
}

#[test]
fn test_knuth_shuffle() {
    let array = thread_rng().gen_iter().take(10).collect::<Vec<f64>>();
    let mut new_array = array.clone();
    knuth_shuffle(&mut new_array);
    assert!(array != new_array);
}

pub mod convex_hull;
