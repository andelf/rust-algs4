use std::fmt;
use std::usize;
use std::iter;

use mtl::array::{Array, ArrayType, concatenate};


/// Simplex solver
pub struct Simplex {
    a: Array<f64>,
    m: usize,
    n: usize,
    basis: Vec<usize>
}

impl fmt::Display for Simplex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (m, n) = (self.m, self.n);
        try!(writeln!(f, "M = {}", m));
        try!(writeln!(f, "N = {}", n));
        for i in 0 .. m+1 {
            for j in 0 .. m+n+1 {
                try!(write!(f, "{:7.2}", self.a[[i,j]]));
            }
            try!(writeln!(f, ""));
        }

        try!(writeln!(f, "value = {}", self.value()));
        for i in 0 .. m {
            if self.basis[i] < n {
                try!(writeln!(f, "x_{} = {}", self.basis[i], self.a[[i,m+n]]));
            }
        }

        Ok(())
    }
}


impl Simplex {
    #[allow(non_snake_case)]
    pub fn new(A: Vec<Vec<f64>>, b: Vec<f64>, c: Vec<f64>) -> Simplex {
        let m = b.len();         // M constrains
        let n = c.len();         // N variables

        let shape = vec![A.len(), A[0].len()];
        for row in A[1..].iter() {
            assert_eq!(row.len(), shape[1]);
        }
        let A = Array::from_vec(A.into_iter().flat_map(|row| row).collect())
            .reshape(shape);
        let b = Array::from_vec(b).reshape([m, 1]);
        let c = Array::from_vec(c).reshape([1, n]);

        let ashape = A.shape();
        let M = concatenate([concatenate([A, Array::eye(ashape[0]),    b], 1),
                             concatenate([c, Array::zeros([1, ashape[0] + 1])], 1)], 0);

        let mut ret = Simplex {
            a: M,
            m: m,
            n: n,
            basis: (0..m).map(|i| n+i).collect()
        };
        ret.solve();
        ret
    }

    /// return optimal objective value
    pub fn value(&self) -> f64 {
        -self.a[[self.m, self.m+self.n]]
    }

    /// primal solution vector
    pub fn primal(&self) -> Vec<f64> {
        let (m, n) = (self.m, self.n);
        let mut x = iter::repeat(0.0).take(n).collect::<Vec<f64>>();
        for i in 0 .. m {
            if self.basis[i] < n {
                x[self.basis[i]] = self.a[[i,m+n]];
            }
        }
        x
    }

    pub fn dual(&self) -> Vec<f64> {
        let (m, n) = (self.m, self.n);
        let mut y = iter::repeat(0.0).take(m).collect::<Vec<f64>>();
        for i in 0 .. m {
            y[i] = -self.a[[m,n+1]];
        }
        y
    }

    /// Bland's rule
    fn bland(&self) -> usize {
        let (m, n) = (self.m, self.n);
        for j in 0 .. m+n {
            if self.a[[m,j]] > 0.0 {
                return j;
            }
        }
        usize::MAX              // optimal :)
    }

    fn min_ratio_rule(&self, q: usize) -> usize {
        // leaving row
        let mut p = usize::MAX;
        let (m, n) = (self.m, self.n);
        for i in 0 .. self.m {
            // skip negative
            if self.a[[i,q]] <= 0.0 {
                continue;
            } else if p == usize::MAX {
                p = i
            } else if self.a[[i,m+n]] / self.a[[i,q]] < self.a[[p,m+n]] / self.a[[p,q]] {
                p = i
            }
        }
        p
    }

    pub fn pivot(&mut self, p: usize, q: usize) {
        let (m, n) = (self.m, self.n);
        for i in 0 .. m+1 {
            for j in 0 .. m+n+1 {
                if i != p && j != q {
                    self.a[[i,j]] -= self.a[[p,j]] * self.a[[i,q]] / self.a[[p,q]];
                }
            }
        }

        // zero out column q
        for i in 0 .. m+1 {
            if i != p { self.a[[i,q]] = 0.0; }
        }

        for j in 0 .. m+n+1 {
            if j != q { self.a[[p,j]] /= self.a[[p,q]] };
        }

        self.a[[p,q]] = 1.0;
    }

    pub fn solve(&mut self) {
        loop {
            let q = self.bland();
            if q == usize::MAX { break }

            let p = self.min_ratio_rule(q);
            if p == usize::MAX {
                panic!("wrong in input question.")
            }

            self.pivot(p, q);

            self.basis[p] = q;
        }
    }

}


#[test]
fn test_simplex_solve() {
    /*
    maximize:  13 * A + 23 * B
    sbject to:
        5 * A + 15 * B <=  480
        4 * A +  4 * B <=  160
        35 * A + 20 * B <= 1190
     */
    let simplex = Simplex::new(
        vec![
            vec![ 5.0, 15.0],
            vec![ 4.0,  4.0],
            vec![35.0, 20.0]
                ],
        vec![480.0, 160.0, 1190.0],
        vec![13.0, 23.0]
            );
    println!("solve => \n{}", simplex);
    assert_eq!(simplex.value(), 800.0);

}
