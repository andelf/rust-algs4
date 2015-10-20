use rand::{thread_rng, Rng};

/// Rearranges an array of objects in uniformly random order
pub fn knuth_shuffle<T>(a: &mut [T]) {
    let mut rng = thread_rng();
    let n = a.len();

    for i in 0 .. n {
        let r = rng.gen_range(0, i+1);
        a.swap(i, r);
    }
}

#[test]
fn test_knuth_shuffle() {
    let array = thread_rng().gen_iter().take(10).collect::<Vec<f64>>();
    let mut new_array = array.clone();
    knuth_shuffle(&mut new_array);
    assert!(array != new_array);
}


pub mod primitive;
pub mod stacks_and_queues;
pub mod union_find;
