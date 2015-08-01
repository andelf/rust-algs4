#![feature(test)]

extern crate test;
extern crate rand;

extern crate algs4;

use test::Bencher;


const M: usize = 3000001;
#[bench]
fn bench_andelf_loop_with_range(b: &mut Bencher) {
    b.iter(|| {
        let mut st = Vec::<Option<i32>>::with_capacity(M);
        for _ in 0..M {
            st.push(None);
        }
        assert!(!st.is_empty());
    });
}


#[bench]
fn bench_andelf_range_then_map(b: &mut Bencher) {
    b.iter(|| {
        let mut st = Vec::<Option<i32>>::with_capacity(M);
        (0..M).map(|_| st.push(None)).count();
        assert!(!st.is_empty());
    });
}



#[bench]
fn bench_andelf_option_unwrap_then_call(b: &mut Bencher) {
    let st = Some("hello world");
    b.iter(|| {
        for _ in 0 .. 1000 {
            assert!(st.unwrap().contains("world"));
        }
    });
}


#[bench]
fn bench_andelf_option_map_then_unwrap(b: &mut Bencher) {
    let st = Some("hello world");
    b.iter(|| {
        for _ in 0 .. 1000 {
            assert!(st.map(|s| s.contains("world")).unwrap());
        }
    });
}
