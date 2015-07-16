use rand::{thread_rng, Rng};

pub struct Ball {
    pub rx: f64,
    pub ry: f64,
    pub vx: f64,
    pub vy: f64,
    pub radius: f64
}


impl Ball {
    pub fn new() -> Ball {
        let mut rng = thread_rng();

        Ball {
            rx: rng.next_f64(),
            ry: rng.next_f64(),
            vx: (rng.next_f64() - 0.5) / 10.0,
            vy: (rng.next_f64() - 0.5) / 10.0,
            radius: (rng.next_f64() + 0.1) / 100.0,
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

    pub fn time_to_hit(&self, that: &Ball) -> f64 {
        unimplemented!()
    }

    pub fn time_to_hit_vertical_wall(&self) -> f64 {
        unimplemented!()
    }

    pub fn time_to_hit_horizontal_wall(&self) -> f64 {
        unimplemented!()
    }
}
