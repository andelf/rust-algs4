// http://coursera.cs.princeton.edu/algs4/assignments/kdtree.html
extern crate algs4;
extern crate rand;

use std::io::prelude::*;
use std::io;

use algs4::geometric_search::primitive::{Point2D, PointSet, RectHV};
use algs4::symbol_tables::ST;
use algs4::geometric_search::kd_tree::KdTree;


fn run() {
    let mut pset = PointSet::new();
    let mut kt: KdTree<Point2D,()> = KdTree::new();

    let npoints = io::BufReader::new(io::stdin())
        .lines()
        .map(|line| line.unwrap().split(' ')
             .map(|xy| xy.parse::<f64>().unwrap())
             .collect::<Vec<f64>>())
        .map(|pt| {
            pset.insert(Point2D::new(pt[0], pt[1]));
            kt.insert(Point2D::new(pt[0], pt[1]));
        })
        .count();

    println!("got {} points", npoints);
    let point: Point2D = Point2D::new(0.9, 0.6);
    let rect = RectHV::new(0.4, 0.4, 0.6, 0.6);

    println!("in rect {}", rect);
    println!("PointSet => {}", pset.range_count(rect));
    println!("KdTree   => {}",   kt.range_count(rect));

    println!("nearest to {}", point);
    println!("PointSet => {:?} d = {}", pset.nearest(point), pset.nearest(point).unwrap().distance_to(point));
    println!("KdTree   => {:?} d = {}", kt.nearest(point), kt.nearest(point).unwrap().distance_to(point));
}


fn main() {
    run();
}
