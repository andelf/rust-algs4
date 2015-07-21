use rand::{thread_rng, Rng, Rand};
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
            radius: (rng.next_f64() + 0.4) / 100.0,
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
            (1.0 - self.rx - self.radius) / self.vx
        } else {
            - (self.rx - self.radius) / self.vx
        }
    }

    pub fn time_to_hit_horizontal_wall(&self) -> f64 {
        if self.vy > 0.0 {
            (1.0 - self.ry - self.radius) / self.vy
        } else {
            - (self.ry - self.radius) / self.vy
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
        self.count += 1;
    }

    pub fn bounce_off_horizontal_wall(&mut self) {
        self.vy = -self.vy;
        self.count += 1;
    }
}


impl Rand for Particle {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        Particle {
            rx: rng.next_f64(),
            ry: rng.next_f64(),
            vx: (rng.next_f64() - 0.5) / 10.0,
            vy: (rng.next_f64() - 0.5) / 10.0,
            mass: rng.next_f64() / 100.0,
            radius: (rng.next_f64() + 0.4) / 100.0,
            count: 0
        }
    }
}


#[derive(PartialEq, Debug)]
pub enum Event {
    Hit { timestamp: f64, a: usize, b: usize, count_a: usize, count_b: usize },
    HitVerticalWall { timestamp: f64, a: usize, count_a: usize },
    HitHorizontalWall { timestamp: f64, a: usize, count_a: usize },
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

    pub fn is_valid_for(&self, particles: &[Particle]) -> bool {
        match *self {
            Event::Hit { a, count_a, b, count_b, .. } => {
                println!("a={:?} cnt={}\nb={:?} cnt={}",
                         particles[a], count_a,
                         particles[b], count_b);
                particles[a].count == count_a && particles[b].count == count_b
            },
            Event::HitVerticalWall { a, count_a, .. } => {
                println!("a={:?} cnt={}", particles[a], count_a);
                particles[a].count == count_a
            },
            Event::HitHorizontalWall { a, count_a, .. } => {
                println!("a={:?} cnt={}", particles[a], count_a);
                particles[a].count == count_a
            },
            Event::Refresh { .. } => true
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
            self.pq.insert(Event::Hit {
                timestamp: self.t + dt,
                a: a, b: i,
                count_a: self.particles[a].count,
                count_b: self.particles[i].count });
        }
        self.pq.insert(Event::HitVerticalWall {
            timestamp: self.t + self.particles[a].time_to_hit_vertical_wall(),
            a: a,
            count_a: self.particles[a].count });
        self.pq.insert(Event::HitHorizontalWall {
            timestamp: self.t + self.particles[a].time_to_hit_horizontal_wall(),
            a: a,
            count_a: self.particles[a].count });
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

        // loop {
        for _ in 0 .. 10 {
            if self.pq.is_empty() {
                break;
            }
            let event = self.pq.del_min().unwrap();

            // FIXME: <= self.t
            if !event.is_valid_for(&self.particles) || event.timestamp() <= self.t {
                continue
            }

            for i in 0 .. n {
                self.particles[i].do_move(event.timestamp() - self.t);
            }
            self.t = event.timestamp();

            match event {
                Event::Hit { a, b, .. } => {
                    println!("{}: {} hit {}", self.t, a, b);
                    let ptr = unsafe { mem::transmute::<_, usize>(&mut self.particles[b]) };
                    self.particles[a].bounce_off(unsafe { mem::transmute::<_, &mut Particle>(ptr) });
                    self.predict(a);
                    self.predict(b);
                },
                Event::HitVerticalWall { a, .. } => {
                    println!("{}: {} hit vertical wall", self.t, a);
                    self.particles[a].bounce_off_vertical_wall();
                    self.predict(a);
                },
                Event::HitHorizontalWall { a, .. } => {
                    println!("{}: {} hit horizontal wall", self.t, a);
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


#[test]
fn test_collision_system() {
    let mut rng = thread_rng();
    let mut ps: Vec<Particle> = Vec::new();
    for _ in 0 .. 20 {
        ps.push(rng.gen());
    }

    let mut sys = CollisionSystem::new(ps);
    sys.simulate();
}
