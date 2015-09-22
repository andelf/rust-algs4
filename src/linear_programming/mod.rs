use mtl::array::{Array, ArrayType, concatenate};


/// Simplex solver
pub struct Simplex {
    a: Array<f64>,
    m: usize,
    n: usize
}


impl Simplex {
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
        let mut M = concatenate([concatenate([A, Array::eye(ashape[0]),    b], 1),
                                 concatenate([c, Array::zeros([1, ashape[0] + 1])], 1)], 0);

        Simplex {
            a: M,
            m: m,
            n: n
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
}
