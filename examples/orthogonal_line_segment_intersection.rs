extern crate algs4;
extern crate rand;

use std::cmp::Ordering;
use rand::{Rng, Rand};
// use rand::thread_rng;

use algs4::sorting::quick_sort;
use algs4::searching::ST;
use algs4::searching::binary_search_tree::BST;
use algs4::geometric_search::RangeSearch1D;

use self::Event::*;
use self::OrthogonalLine::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OrthogonalLine {
    HLine { x0: f64, x1: f64, y: f64 },
    VLine { x: f64, y0: f64, y1: f64 }
}

impl OrthogonalLine {
    fn to_events(&self) -> Vec<Event> {
        match self {
            &HLine { x0, x1, y } => vec!
                [HLineStart { x: x0, y: y },
                 HLineEnd   { x: x1, y: y }],
            &VLine { x, y0, y1 } => vec![VLineMet { x: x, y0: y0, y1: y1 }]
        }
    }
}

impl Rand for OrthogonalLine {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        if rng.gen_weighted_bool(3) {
            VLine { x:  rng.gen_range(0.0, 10.0),
                    y0: rng.gen_range(0.0, 10.0),
                    y1: rng.gen_range(0.0, 10.0),
            }
        } else {
            HLine { x0: rng.gen_range(0.0, 10.0),
                    x1: rng.gen_range(0.0, 10.0),
                    y:  rng.gen_range(0.0, 10.0),
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Event {
    HLineStart { x: f64, y: f64 },
    HLineEnd { x: f64, y: f64 },
    VLineMet { x: f64, y0: f64, y1: f64 }
}

fn first_x_coords_of(event: &Event) -> f64 {
    match *event {
        HLineStart { x, .. } => x,
        HLineEnd { x, .. }   => x,
        VLineMet { x, .. }   => x
    }
}
impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Event) -> Option<Ordering> {
        first_x_coords_of(self).partial_cmp(&first_x_coords_of(other))
    }
}



fn lines_from_slide() -> Vec<OrthogonalLine> {
    vec![
        HLine { x0: 2.0, x1: 14.5, y: 11.0 },
        HLine { x0: 3.0, x1: 9.5, y: 8.0 },
        HLine { x0: 4.0, x1: 6.0, y: 6.5 },
        HLine { x0: 4.0, x1: 6.0, y: 6.5 },
        HLine { x0: 5.0, x1: 12.0, y: 3.0 },
        HLine { x0: 5.0, x1: 12.0, y: 3.0 },
        HLine { x0: 10.0, x1: 17.0, y: 6.0 },
        HLine { x0: 12.0, x1: 19.0, y: 7.5 },
        HLine { x0: 15.0, x1: 18.0, y: 9.0 },
        HLine { x0: 15.0, x1: 17.5, y: 3.0 },
        VLine { x: 8.0, y0: 5.0, y1: 9.0 },
        VLine { x: 14.0, y0: 10.5, y1: 12.0 },
        VLine { x: 15.5, y0: 1.0, y1: 6.0 },
        VLine { x: 20.0, y0: 2.0, y1: 12.0 },
        ]
}

fn main() {
    let mut bst = BST::<f64,()>::new();
    let lines = lines_from_slide();
    let mut events: Vec<Event> = lines.iter().flat_map(|l| l.to_events()).collect();

    quick_sort(&mut events);
    for e in events {
        match e {
            HLineStart { y, .. } => bst.insert(y, ()),
            HLineEnd { y, .. } => bst.delete(&y),
            VLineMet { x, y0, y1 } => {
                println!("vline => ({},{})---({},{})", x, y0, x, y1);
                match bst.range_search(&y0, &y1) {
                    None => println!("no intersection"),
                    Some(ys) => for y in ys {
                        println!("intersection: ({}, {})", x, *y);
                    }
                }
            }
        }
    }
}
