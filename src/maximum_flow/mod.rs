use std::fmt;
use std::iter;

use adivon::bag::Bag;


/// Capacitated edge with a flow in a flow network.
#[derive(Clone, Copy)]
pub struct FlowEdge {
    v: usize,                   // from
    w: usize,                   // to
    capacity: f64,
    flow: f64
}

impl FlowEdge {
    pub fn new_zero_flow(v: usize, w: usize, capacity: f64) -> FlowEdge {
        assert!(capacity >= 0.0, "edge capacity must be non-negative");
        FlowEdge {
            v: v,
            w: w,
            capacity: capacity,
            flow: 0.0
        }
    }

    pub fn new(v: usize, w: usize, capacity: f64, flow: f64) -> FlowEdge {
        assert!(capacity >= 0.0, "edge capacity must be non-negative");
        assert!(flow <= capacity, "flow exceeds capacity");
        assert!(flow >= 0.0, "flow must be non-negative");
        FlowEdge {
            v: v,
            w: w,
            capacity: capacity,
            flow: flow
        }
    }

    pub fn from(&self) -> usize {
        self.v
    }

    pub fn to(&self) -> usize {
        self.w
    }

    pub fn capacity(&self) -> f64 {
        self.capacity
    }

    pub fn flow(&self) -> f64 {
        self.flow
    }

    pub fn other(&self, vertex: usize) -> usize {
        if vertex == self.v {
            self.w
        } else if vertex == self.w {
            self.v
        } else {
            panic!("illegal endpoint")
        }
    }

    pub fn residual_capacity_to(&self, vertex: usize) -> f64 {
        if vertex == self.v {
            self.flow
        } else if vertex == self.w {
            self.capacity - self.flow
        } else {
            panic!("illegal endpoint")
        }
    }

    pub fn add_residual_flow_to(&mut self, vertex: usize, delta: f64) {
        assert!(!delta.is_nan(), "change in flow = NaN");
        if vertex == self.v {
            self.flow -= delta;
        } else if vertex == self.w {
            self.flow += delta;
        } else {
            panic!("illegal endpoint")
        }
        assert!(self.flow >= 0.0 && self.flow <= self.capacity)
    }
}

impl fmt::Debug for FlowEdge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {} {}/{}", self.v, self.w, self.flow, self.capacity)
    }
}

#[test]
fn test_flow_edge() {
    let e = FlowEdge::new_zero_flow(12, 23, 3.14);
    assert_eq!("12 -> 23 0/3.14", format!("{:?}", e));
}

/// A capacitated flow network, implemented using adjacency lists.
pub struct FlowNetwork {
    v: usize,
    e: usize,
    adj: Vec<Bag<FlowEdge>>
}

impl FlowNetwork {
    pub fn new(v: usize) -> FlowNetwork {
        FlowNetwork {
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

    fn validate_vertex(&self, v: usize) {
        assert!(v < self.v())
    }

    pub fn add_edge(&mut self, e: FlowEdge) {
        let v = e.from();
        let w = e.to();
        self.validate_vertex(v);
        self.validate_vertex(w);
        self.adj[v].add(e);
        self.adj[w].add(e);
        self.e += 1;
    }

    pub fn adj(&self, v: usize) -> ::std::vec::IntoIter<FlowEdge> {
        self.validate_vertex(v);
        self.adj[v].iter().map(|e| e.clone()).collect::<Vec<FlowEdge>>().into_iter()
    }

    pub fn edges(&self) -> ::std::vec::IntoIter<FlowEdge> {
        self.adj.iter()
            .flat_map(|adj| {
                adj.iter().map(|e| e.clone()).collect::<Vec<FlowEdge>>().into_iter()
            })
            .collect::<Vec<FlowEdge>>()
            .into_iter()
    }
}
