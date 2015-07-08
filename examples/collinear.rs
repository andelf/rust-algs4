// http://coursera.cs.princeton.edu/algs4/assignments/collinear.html
extern crate algs4;

use std::io::prelude::*;
use std::io;
use std::fmt;
use std::cmp::Ordering;

use algs4::quicksort::quick_sort;
use algs4::mergesort::comparator::Comparator;
use algs4::mergesort::comparator::insertion_sort;

#[derive(Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

pub struct SlopeOrder<'a> {
    pt: &'a Point
}

impl<'a> Comparator<Point> for SlopeOrder<'a> {
    fn compare(&self, q1: &Point, q2: &Point) -> Ordering {
        // unsafe to refer :(
        self.pt.slope_to(q1).partial_cmp(&self.pt.slope_to(q2)).unwrap()
    }
}


/// represents a point in the plane
impl Point {
    /// construct the point (x, y)
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    /// draw this point
    pub fn draw(&self) {
        unimplemented!()
    }

    /// draw the line segment from this point to that point
    pub fn draw_to(&self, _other: &Point) {
        unimplemented!()
    }

    /// the slope between this point and that point
    pub fn slope_to(&self, other: &Point) -> f64 {
        ((other.y - self.y) as f64) / ((other.x - self.x) as f64)
    }

    #[allow(non_snake_case)]
    pub fn SLOPE_ORDER<'a>(&'a self) -> SlopeOrder<'a> {
        SlopeOrder { pt: self }
    }
}

impl PartialEq<Point> for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}


// is this point lexicographically smaller than that point?
impl PartialOrd<Point> for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        if self.y < other.y || (self.y == other.y && self.x < other.x) {
            Some(Ordering::Less)
        } else if self.x == other.x && self.y == other.y {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Greater)
        }
    }
}


impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "({}, {})", self.x, self.y));
        Ok(())
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "({}, {})", self.x, self.y));
        Ok(())
    }
}


// examines 4 points at a time and checks whether they all lie on the same line segment,
// printing out any such line segments to standard output and drawing them using standard drawing.
// To check whether the 4 points p, q, r, and s are collinear, check whether the slopes between
// p and q, between p and r, and between p and s are all equal.
fn brute_force(points: &[Point]) {
    for p in points.iter() {
        for q in points.iter() {
            if p == q { break }
            for r in points.iter() {
                if r == p || r == q { break }
                for s in points.iter() {
                    if s == p || s == q || s == r { break }
                    let slope = p.slope_to(q);
                    if slope == p.slope_to(r) && slope == p.slope_to(s) {
                        println!("{} -> {} -> {} -> {}", p, q, r, s);
                    }
                }
            }
        }
    }
}

// A faster, sorting-based solution. Remarkably, it is possible to solve the problem much faster
// than the brute-force solution described above. Given a point p, the following method determines
// whether p participates in a set of 4 or more collinear points.
// - Think of p as the origin.
// - For each other point q, determine the slope it makes with p.
// - Sort the points according to the slopes they makes with p.
// - Check if any 3 (or more) adjacent points in the sorted order have equal slopes with respect
//   to p. If so, these points, together with p, are collinear.
fn sorting_based(points: &[Point]) {
    let n = points.len();

    let mut points = points.to_vec();
    assert!(n > 4);
    let mut result: Vec<Vec<Point>> = Vec::new();

    for i in 1 .. n {
        // 0th is the origin
        points.swap(0, i);

        let mut points = points.clone();
        let p = points[0].clone();

        //
        quick_sort(&mut points[1..]);
        // insertion sort is stable
        insertion_sort(&mut points[1..], p.SLOPE_ORDER());

        let mut n = 0usize;
        // always 0th item
        let mut coll_points_idx = vec![0];
        let mut prev_slope = 99999.9;

        for (idx, slope) in points.iter().map(|q| p.slope_to(q)).enumerate() {
            if idx == 0 {
                continue;
            }
            if slope == prev_slope {
                n += 1;
                coll_points_idx.push(idx);
            } else {
                // if more than 4 points in seq
                if n >= 4 {
                    let mut line: Vec<Point> = coll_points_idx.iter().map(|&i| points[i].clone()).collect();
                    quick_sort(&mut line);
                    if !result.contains(&line) {
                        result.push(line)
                    }
                }
                // every time we went here, we already have 2 points.
                n = 2;
                coll_points_idx = vec![0, idx];
            }
            prev_slope = slope;
        }
        // FIXME: duplicated logic
        if n >= 4 {
            let mut line: Vec<Point> = coll_points_idx.iter().map(|&i| points[i].clone()).collect();
            quick_sort(&mut line);
            if !result.contains(&line) {
                result.push(line)
            }
        }
    }
    for line in result.iter() {
        let desc = line.iter().map(|i| format!("{}", i)).collect::<Vec<String>>().connect(" -> ");
        println!("{}", desc);

    }
}


fn main() {
    let mut lines = io::BufReader::new(io::stdin()).lines();
    let n = lines.next().unwrap().unwrap().parse().unwrap();

    let mut points: Vec<Point> = Vec::new();
    for _ in 0 .. n {
        let segs: Vec<i32> = lines.next().unwrap().unwrap().split(' ').
            filter(|s| !s.is_empty()).map(|n| n.parse().unwrap()).collect();
        let x = segs[0];
        let y = segs[1];

        // println!("x = {} y = {}", x, y);
        points.push(Point::new(x, y));
    }

    println!("# Brute force");
    brute_force(&points);

    println!("# A faster, sorting-based solution");
    sorting_based(&mut points);
}
