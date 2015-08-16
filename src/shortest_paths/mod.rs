use std::fmt;
use std::iter;

use adivon::bag::Bag;

/// Weighted directed edge
#[derive(Clone, Copy)]
pub struct DirectedEdge {
    v: usize,
    w: usize,
    weight: f64
}

impl DirectedEdge {
    pub fn new(v: usize, w: usize, weight: f64) -> DirectedEdge {
        assert!(!weight.is_nan(), "weight is NaN");
        DirectedEdge {
            v: v,
            w: w,
            weight: weight
        }
    }

    #[inline]
    pub fn from(&self) -> usize {
        self.v
    }

    #[inline]
    pub fn to(&self) -> usize {
        self.w
    }

    #[inline]
    pub fn weight(&self) -> f64 {
        self.weight
    }
}

impl fmt::Debug for DirectedEdge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {} {:5.2}", self.v, self.w, self.weight)
    }
}

#[test]
fn test_directed_edge() {
    let e = DirectedEdge::new(12, 24, 3.14);
    assert_eq!(format!("{:?}", e), "12 -> 24  3.14");
}


/// Edge-weighted digraph, implemented using adjacency lists
#[derive(Clone)]
pub struct EdgeWeightedDigraph {
    v: usize,
    e: usize,
    adj: Vec<Bag<DirectedEdge>>
}

impl EdgeWeightedDigraph {
    pub fn new(v: usize) -> EdgeWeightedDigraph {
        EdgeWeightedDigraph {
            v: v,
            e: 0,
            adj: iter::repeat(Bag::new()).take(v).collect()
        }
    }

    pub fn v(&self) -> usize {
        self.v
    }

    pub fn e(&self) -> usize {
        self.e
    }

    #[inline]
    fn validate_vertex(&self, v: usize) {
        assert!(v < self.v, "vertex must be between 0 and V");
    }

    pub fn add_edge(&mut self, e: DirectedEdge) {
        let v = e.from();
        let w = e.to();
        self.validate_vertex(v);
        self.validate_vertex(w);
        self.adj[v].add(e);
        self.e += 1;
    }

    pub fn adj(&self, v: usize) -> ::std::vec::IntoIter<DirectedEdge> {
        self.validate_vertex(v);
        self.adj[v].iter().map(|e| e.clone()).collect::<Vec<DirectedEdge>>().into_iter()
    }

    pub fn outdegree(&self, v: usize) -> usize {
        self.validate_vertex(v);
        self.adj[v].len()
    }

    pub fn edges(&self) -> ::std::vec::IntoIter<DirectedEdge> {
        self.adj.iter()
            .flat_map(|adj| {
                adj.iter().map(|e| e.clone()).collect::<Vec<DirectedEdge>>().into_iter()
            })
            .collect::<Vec<DirectedEdge>>()
            .into_iter()
    }

    pub fn to_dot(&self) -> String {
        let mut dot = String::new();

        dot.push_str("digraph G {\n");
        for i in 0 .. self.v {
            dot.push_str(&format!("  {};\n", i));
        }

        for e in self.edges() {
            let v = e.from();
            let w = e.to();
            dot.push_str(&format!("  {} -> {} [ label=\"{}\" ];\n",
                                  v, w, e.weight))
        }
        dot.push_str("}\n");
        dot
    }
}
