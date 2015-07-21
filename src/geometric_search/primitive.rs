use std::fmt;


#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Point2D {
    pub x: f64,
    pub y: f64
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Point2D {
        Point2D { x: x, y: y }
    }

    pub fn distance_to<T: AsRef<Point2D>>(&self, that: T) -> f64 {
        self.distance_squared_to(that).sqrt()
    }

    pub fn distance_squared_to<T: AsRef<Point2D>>(&self, that: T) -> f64 {
        (self.x - that.as_ref().x).powi(2) + (self.y - that.as_ref().y).powi(2)
    }
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl AsRef<Point2D> for Point2D {
    fn as_ref(&self) -> &Point2D {
        &self
    }
}

#[test]
fn test_point2d() {
    let p1 = Point2D::new(0.0, 0.0);
    let p2 = Point2D::new(1.0, 1.0);

    // maybe bad :(
    assert_eq!(p1.distance_to(p2), (2.0f64).sqrt());
}


#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
/// Implementation of 2D axis-aligned rectangle
pub struct RectHV {
    pub xmin: f64,
    pub ymin: f64,
    pub xmax: f64,
    pub ymax: f64
}

impl RectHV {
    pub fn new(xmin: f64, ymin: f64, xmax: f64, ymax: f64) -> RectHV {
        RectHV { xmin: xmin, ymin: ymin, xmax: xmax, ymax: ymax }
    }

    pub fn width(&self) -> f64 {
        self.xmax - self.xmin
    }

    pub fn height(&self) -> f64 {
        self.ymax - self.ymin
    }

    pub fn contains<T: AsRef<Point2D>>(&self, p: T) -> bool {
        let p = p.as_ref();
        p.x >= self.xmin && p.y >= self.ymin &&
            p.x <= self.xmax && p.y <= self.ymax
    }

    /// does this axis-aligned rectangle intersect that one?
    pub fn intersects<T: AsRef<RectHV>>(&self, that: T) -> bool {
        let that = that.as_ref();
        self.xmax >= that.xmin && self.ymax >= that.ymin &&
            that.xmax >= self.xmin && that.ymax >= self.ymin
    }

    /// distance from p to closest point on this axis-aligned rectangle
    pub fn distance_to<T: AsRef<Point2D>>(&self, p: T) -> f64 {
        self.distance_squared_to(p).sqrt()
    }

    /// distance squared from p to closest point on this axis-aligned rectangle
    pub fn distance_squared_to<T: AsRef<Point2D>>(&self, p: T) -> f64 {
        let p = p.as_ref();
        let mut dx = 0.0;
        let mut dy = 0.0;
        if p.x < self.xmin {
            dx = p.x - self.xmin;
        } else if p.x > self.xmax {
            dx = p.x - self.xmax;
        }
        if p.y < self.ymin {
            dy = p.y - self.ymin;
        } else if p.y > self.ymax {
            dy = p.y - self.ymax;
        }
        dx.powi(2) + dy.powi(2)
    }
}

impl fmt::Display for RectHV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}] x [{}, {}]", self.xmin, self.xmax, self.ymin, self.ymax)
    }
}


impl AsRef<RectHV> for RectHV {
    fn as_ref(&self) -> &RectHV {
        &self
    }
}

#[test]
fn test_rect() {
    let r1 = RectHV::new(0.0, 0.0, 1.1, 1.1);
    let r2 = RectHV::new(1.2, 2.0, 3.1, 4.1);
    assert!(!r1.intersects(r2));
}
