#![feature(test)]

extern crate test;
extern crate rand;

extern crate algs4;

use test::Bencher;
use rand::{thread_rng, Rng};

use algs4::searching::ST;
use algs4::fundamentals::primitive::{Point2D, PointSet, RectHV};
use algs4::geometric_search::kd_tree::KdTree;

const SIZE: usize = 1000;


#[bench]
fn bench_brute_force_range_search(b: &mut Bencher) {
    let mut pset = PointSet::new();
    let mut rng = thread_rng();
    for _ in 0 .. SIZE {
        pset.insert(rng.gen());
    }
    let rect = RectHV::new(0.4, 0.4, 0.6, 0.6);
    b.iter(|| {
        assert!(pset.range_search(&rect).next().is_some());
    });
}


#[bench]
fn bench_kd_tree_range_search(b: &mut Bencher) {
    let mut kt = KdTree::new();
    let mut rng = thread_rng();
    for _ in 0 .. SIZE {
        kt.insert(rng.gen());
    }
    let rect = RectHV::new(0.4, 0.4, 0.6, 0.6);
    b.iter(|| {
        assert!(kt.range_search(&rect).next().is_some());
    });
}


#[bench]
fn bench_brute_force_nearest(b: &mut Bencher) {
    let mut pset = PointSet::new();
    let mut rng = thread_rng();
    for _ in 0 .. SIZE {
        pset.insert(rng.gen());
    }
    let p = Point2D::new(0.5, 0.5);
    b.iter(|| {
        assert!(pset.nearest(&p).is_some());
    });
}


#[bench]
fn bench_kd_tree_nearest(b: &mut Bencher) {
    let mut kt = KdTree::new();
    let mut rng = thread_rng();
    for _ in 0 .. SIZE {
        kt.insert(rng.gen());
    }
    let p = Point2D::new(0.5, 0.5);
    b.iter(|| {
        assert!(kt.nearest(&p).is_some());
    });
}
