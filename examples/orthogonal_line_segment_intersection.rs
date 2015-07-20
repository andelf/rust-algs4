extern crate algs4;
extern crate rand;

use std::cmp::Ordering;
use rand::{Rng, Rand};
// use rand::thread_rng;

use algs4::quicksort::quick_sort;
use algs4::symbol_tables::ST;
use algs4::symbol_tables::binary_search_tree::BST;
use algs4::geometric_search::RangeSearch1D;

use self::Event::*;
use self::OrthogonalLine::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OrthogonalLine {
    HLine { x0: i32, x1: i32, y: i32 },
    VLine { x: i32, y0: i32, y1: i32 }
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
            VLine { x:  rng.gen_range(0, 100),
                    y0: rng.gen_range(0, 100),
                    y1: rng.gen_range(0, 100),
            }
        } else {
            HLine { x0: rng.gen_range(0, 100),
                    x1: rng.gen_range(0, 100),
                    y:  rng.gen_range(0, 100),
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Event {
    HLineStart { x: i32, y: i32 },
    HLineEnd { x: i32, y: i32 },
    VLineMet { x: i32, y0: i32, y1: i32 }
}

fn first_x_coords_of(event: &Event) -> i32 {
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
        HLine { x0: 20, x1: 145, y: 110 },
        HLine { x0: 30, x1: 95, y: 80 },
        HLine { x0: 40, x1: 60, y: 65 },
        HLine { x0: 40, x1: 60, y: 65 },
        HLine { x0: 50, x1: 120, y: 30 },
        HLine { x0: 50, x1: 120, y: 30 },
        HLine { x0: 100, x1: 170, y: 60 },
        HLine { x0: 120, x1: 190, y: 75 },
        HLine { x0: 150, x1: 180, y: 90 },
        HLine { x0: 150, x1: 175, y: 30 },
        VLine { x: 80, y0: 50, y1: 90 },
        VLine { x: 140, y0: 105, y1: 120 },
        VLine { x: 155, y0: 10, y1: 60 },
        VLine { x: 200, y0: 20, y1: 120 },
        ]
}

fn main() {
    let mut bst = BST::<i32,()>::new();
    let lines = lines_from_slide();
    let mut events: Vec<Event> = lines.iter().flat_map(|l| l.to_events()).collect();

    quick_sort(&mut events);
    for e in events {
        match e {
            HLineStart { y, .. } => bst.insert(y, ()),
            HLineEnd { y, .. } => bst.delete(&y),
            VLineMet { x, y0, y1 } => {
                println!("vline => ({},{})---({},{})", x, y0, x, y1);
                match bst.search(&y0, &y1) {
                    None => println!("no intersection"),
                    Some(ys) => for y in ys {
                        println!("intersection: ({}, {})", x, *y);
                    }
                }
            }
        }
    }
}
