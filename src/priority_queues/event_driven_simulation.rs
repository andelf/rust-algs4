use rand::{thread_rng, Rng};
use std::f64;
use std::cmp::Ordering;
use std::mem;
use super::MinPQ;
use super::binary_heaps::BinaryHeapMinPQ;

#[derive(PartialEq, Debug)]
pub struct Particle {
    pub rx: f64,
    pub ry: f64,
    pub vx: f64,
    pub vy: f64,
    pub mass: f64,
    pub radius: f64,
    count: usize
}


impl Particle {
    pub fn new() -> Particle {
        let mut rng = thread_rng();

        Particle {
            rx: rng.next_f64(),
            ry: rng.next_f64(),
            vx: (rng.next_f64() - 0.5) / 10.0,
            vy: (rng.next_f64() - 0.5) / 10.0,
            mass: rng.next_f64() / 100.0,
            radius: (rng.next_f64() + 0.3) / 100.0,
            count: 0
        }
    }

    pub fn do_move(&mut self, dt: f64) {
        // collision with walls
        let (rx, ry, vx, vy) = (self.rx, self.ry, self.vx, self.vy);
        let radius = self.radius;
        if rx + vx*dt < radius || rx + vx*dt > 1.0 - radius {
            self.vx = -vx;
        }
        if ry + vy*dt < radius || ry + vy*dt > 1.0 - radius {
            self.vy = -vy;
        }
        self.rx = rx + vx*dt;
        self.ry = ry + vy*dt;
    }

    pub fn time_to_hit(&self, that: &Particle) -> f64 {
        if self == that {
            return f64::INFINITY
        }
        let dx = that.rx - self.rx;
        let dy = that.ry - self.ry;

        let dvx = that.vx - self.vx;
        let dvy = that.vy - self.vy;

        let dvdr = dx*dvx + dy*dvy;

        if dvdr > 0.0 {
            return f64::INFINITY;
        }

        let dvdv = dvx*dvx + dvy*dvy;
        let drdr = dx*dx + dy*dy;
        let sigma = self.radius + that.radius;
        let d = dvdr*dvdr - dvdv * (drdr - sigma*sigma);
        if d < 0.0 {
            return f64::INFINITY;
        }

        return - (dvdr + d.sqrt()) / dvdv
    }

    pub fn time_to_hit_vertical_wall(&self) -> f64 {
        if self.vx > 0.0 {
            (1.0 - self.rx) / self.vx
        } else {
            - self.rx / self.vx
        }
    }

    pub fn time_to_hit_horizontal_wall(&self) -> f64 {
        if self.vy > 0.0 {
            (1.0 - self.ry) / self.vy
        } else {
            - self.ry / self.vy
        }
    }

    pub fn bounce_off(&mut self, that: &mut Particle) {
        let dx = that.rx - self.rx;
        let dy = that.ry - self.ry;
        let dvx = that.vx - self.vx;
        let dvy = that.vy - self.vy;
        let dvdr = dx*dvx + dy*dvy;
        let dist = self.radius + that.radius;
        let j = 2.0 * self.mass * that.mass * dvdr / ((self.mass + that.mass) * dist);

        let jx = j*dx / dist;
        let jy = j*dy / dist;

        self.vx += jx / self.mass;
        self.vy += jy / self.mass;
        that.vx -= jx / that.mass;
        that.vy -= jy / that.mass;

        self.count += 1;
        that.count += 1;
    }

    pub fn bounce_off_vertical_wall(&mut self) {
        self.vx = -self.vx;
    }

    pub fn bounce_off_horizontal_wall(&mut self) {
        self.vy = -self.vy;
    }
}


#[derive(PartialEq)]
pub enum Event {
    Hit { timestamp: f64, a: usize, b: usize },
    HitVerticalWall { timestamp: f64, a: usize },
    HitHorizontalWall { timestamp: f64, a: usize },
    Refresh { timestamp: f64 }
}

impl Event {
    pub fn timestamp(&self) -> f64 {
        match self {
            &Event::Hit { timestamp: t, .. } => t,
            &Event::HitVerticalWall { timestamp: t, .. } => t,
            &Event::HitHorizontalWall { timestamp: t, .. } => t,
            &Event::Refresh { timestamp: t } => t
        }
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Event) -> Option<Ordering> {
        self.timestamp().partial_cmp(&other.timestamp())
    }
}

pub struct CollisionSystem {
    pq: BinaryHeapMinPQ<Event>,
    t: f64,
    particles: Vec<Particle>
}

impl CollisionSystem {
    pub fn new(particles: Vec<Particle>) -> CollisionSystem {
        CollisionSystem {
            pq: BinaryHeapMinPQ::new(),
            t: 0.0,
            particles: particles
        }
    }

    fn predict(&mut self, a: usize) {
        // TODO move events in PQ out
        for i in 0 .. self.particles.len() {
            if i == a {
                continue;
            }
            let dt = self.particles[a].time_to_hit(&self.particles[i]);
            self.pq.insert(Event::Hit { timestamp: self.t + dt,
                                        a: a, b: i });
        }
        self.pq.insert(Event::HitVerticalWall { timestamp: self.t + self.particles[a].time_to_hit_vertical_wall(),
                                                a: a });
        self.pq.insert(Event::HitHorizontalWall { timestamp: self.t + self.particles[a].time_to_hit_horizontal_wall(),
                                                  a: a });
    }

    // FIXME: un-done
    // TODO: add message remove or valid check
    pub fn simulate(&mut self) {
        self.pq = BinaryHeapMinPQ::new();

        let n = self.particles.len();
        for i in 0 .. n {
            self.predict(i);
        }
        self.pq.insert(Event::Refresh { timestamp: 0.0 });

        loop {
            if self.pq.is_empty() {
                break;
            }
            let event = self.pq.del_min().unwrap();

            // if !event.is_valid() {
            //     continue;
            // }

            for i in 0 .. n {
                self.particles[i].do_move(event.timestamp() - self.t);
            }
            self.t = event.timestamp();

            match event {
                Event::Hit { a, b, .. } => {
                    let ptr = unsafe { mem::transmute::<_, usize>(&mut self.particles[b]) };
                    self.particles[a].bounce_off(unsafe { mem::transmute::<_, &mut Particle>(ptr) });
                    self.predict(a);
                    self.predict(b);
                },
                Event::HitVerticalWall { a, .. } => {
                    self.particles[a].bounce_off_vertical_wall();
                    self.predict(a);
                },
                Event::HitHorizontalWall { a, .. } => {
                    self.particles[a].bounce_off_horizontal_wall();
                    self.predict(a);
                },
                Event::Refresh { .. } => {
                    println!("refresh");
                }
            }

        }
    }

}
