use std::cmp::Ordering;
use std::mem;
use super::mergesort::comparator::Comparator;

#[allow(non_snake_case)]
pub struct Point2D {
    x: f64,
    y: f64,
    pub POLAR_ORDER: PolarOrder,
}

pub struct PolarOrder {
    _dummy: f64,              // dummy position
}



// TODO
impl Comparator<Point2D> for PolarOrder {
    fn compare(&self, q1: &Point2D, q2: &Point2D) -> Ordering {

        // ugly hack
        let p: &Point2D = unsafe { mem::transmute((&self._dummy as *const f64).offset(-2)) };

        let y = p.y;
        let dy1 = q1.y - y;
        let dy2 = q2.y - y;

        if dy1 == 0f64 && dy2 == 0f64 {
            // p, q1, q2 horizontal
            unimplemented!()
        } else if dy1 >= 0f64 && dy2 < 0f64 {
            // q1 above p; q2 below p
            Ordering::Less
        } else if dy2 >= 0f64 && dy1 < 0f64 {
            // q1 below p; q2 above p
            Ordering::Greater
        } else {
            // FIXME: type mismatch
            // -Point2D::ccw(self.p, q1, q2)
            unimplemented!()
        }
    }
}


impl Point2D {
    pub fn new(x: f64, y: f64) -> Point2D {
        Point2D { x: x, y: y,
                  POLAR_ORDER: PolarOrder { _dummy: 0.0}
        }
    }

    pub fn ccw(a: &Point2D, b: &Point2D, c: &Point2D) -> i32 {
        let area2 = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
        match area2 {
            _ if area2 < 0f64 => -1,
            _ if area2 > 0f64 =>  1,
            _                 =>  0
        }
    }
}


#[test]
fn test_point2d() {
    let p1 = Point2D::new(0.5, 0.6);
    let p2 = Point2D::new(1.0, 3.0);
    let p3 = Point2D::new(-2.0, -1.0);

    p1.POLAR_ORDER.compare(&p2, &p3);
}
