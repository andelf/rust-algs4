use std::fmt;
use std::iter;
use std::f64;
use std::sync::{Arc, RwLock};

use adivon::bag::Bag;
use adivon::queue::Queue;


/// Capacitated edge with a flow in a flow network.
pub struct FlowEdge {
    v: usize,                   // from
    w: usize,                   // to
    capacity: f64,
    flow: f64
}

impl FlowEdge {
    pub fn new(v: usize, w: usize, capacity: f64) -> FlowEdge {
        assert!(capacity >= 0.0, "edge capacity must be non-negative");
        FlowEdge {
            v: v,
            w: w,
            capacity: capacity,
            flow: 0.0
        }
    }

    pub fn new_with_flow(v: usize, w: usize, capacity: f64, flow: f64) -> FlowEdge {
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
    let e = FlowEdge::new(12, 23, 3.14);
    assert_eq!("12 -> 23 0/3.14", format!("{:?}", e));
}

/// A capacitated flow network, implemented using adjacency lists.
pub struct FlowNetwork {
    v: usize,
    e: usize,
    adj: Vec<Bag<Arc<RwLock<FlowEdge>>>>
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
        let edge = Arc::new(RwLock::new(e));
        self.adj[v].add(edge.clone());
        self.adj[w].add(edge);
        self.e += 1;
    }

    pub fn adj<'r>(&'r self, v: usize) -> Vec<Arc<RwLock<FlowEdge>>> {
        self.validate_vertex(v);
        self.adj[v].iter().map(|e| e.clone()).collect()
    }

    pub fn edges<'r>(&'r self) -> Vec<Arc<RwLock<FlowEdge>>> {
        self.adj.iter()
            .flat_map(|adj| {
                adj.iter()
                    .map(|e| e.clone())
                    .collect::<Vec<Arc<RwLock<FlowEdge>>>>()
                    .into_iter()
            })
            .collect()
    }

    pub fn to_dot(&self) -> String {
        let mut dot = String::new();

        dot.push_str("digraph G {\n");
        for i in 0 .. self.v {
            dot.push_str(&format!("  {};\n", i));
        }

        for v in 0 .. self.v() {
            for e in self.adj(v) {
                let e = e.read().unwrap();
                if v == e.from() {
                    let w = e.to();
                    dot.push_str(&format!("  {} -> {} [ label=\"{}/{}\" ];\n",
                                          v, w, e.flow, e.capacity));
                }
            }
        }
        dot.push_str("}\n");
        dot
    }
}

/// Ford-Fulkerson algorithm for computing a max flow and
/// a min cut using shortest augmenting path rule.
pub struct FordFulkerson<'g> {
    graph: &'g FlowNetwork,
    marked: Vec<bool>,
    edge_to: Vec<Option<Arc<RwLock<FlowEdge>>>>,
    value: f64
}

impl<'g> FordFulkerson<'g> {
    fn new<'a>(graph: &'a FlowNetwork, s: usize, t: usize) -> FordFulkerson<'a> {
        graph.validate_vertex(s);
        graph.validate_vertex(t);

        let mut ret = FordFulkerson {
            graph: graph,
            marked: Vec::new(), // will init in has_augmenting_path()
            edge_to: Vec::new(), // as above
            value: 0.0
        };
        ret.ford_fulkerson(s, t);
        ret
    }

    fn ford_fulkerson(&mut self, s: usize, t: usize) {
        if !self.is_feasible(s, t) {
            panic!("initial flow is infeasible");
        }

        self.value = self.excess(t);
        while self.has_augmenting_path(s, t) {
            let mut bottle = f64::INFINITY;
            let mut v = t;
            while v != s {
                let ee = self.edge_to[v].as_ref().unwrap().clone();
                bottle = bottle.min(ee.write().unwrap().residual_capacity_to(v));
                v = ee.read().unwrap().other(v);
            }

            // augment flow
            let mut v = t;
            while v != s {
                let ee = self.edge_to[v].as_ref().unwrap().clone();
                ee.write().unwrap().add_residual_flow_to(v, bottle);
                v = ee.read().unwrap().other(v);
            }

            self.value += bottle;
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    // is v in the s side of the min s-t cut?
    pub fn in_cut(&self, v: usize) -> bool {
        self.graph.validate_vertex(v);
        self.marked[v]
    }

    // is there an augmenting path?
    fn has_augmenting_path(&mut self, s: usize, t: usize) -> bool {
        self.edge_to = iter::repeat(None).take(self.graph.v()).collect();
        self.marked = iter::repeat(false).take(self.graph.v()).collect();

        let mut queue = Queue::<usize>::new();
        queue.enqueue(s);
        self.marked[s] = true;
        while !queue.is_empty() && !self.marked[t] {
            let v = queue.dequeue().unwrap();

            for ee in self.graph.adj(v) {
                let e = ee.read().unwrap();
                let w = e.other(v);

                // if residual capacity from v to w
                if e.residual_capacity_to(w) > 0.0 {
                    if !self.marked[w] {
                        self.edge_to[w] = Some(ee.clone());
                        self.marked[w] = true;
                        queue.enqueue(w);
                    }
                }
            }
        }
        self.marked[t]
    }

    // return excess flow at vertex v
    fn excess(&self, v: usize) -> f64 {
        let mut excess = 0f64;
        for e in self.graph.adj(v) {
            let e = e.read().unwrap();
            if v == e.from() {
                excess -= e.flow();
            } else {
                excess += e.flow();
            }
        }
        excess
    }

    fn is_feasible(&self, s: usize, t: usize) -> bool {
        const EPSILON: f64 = 1e-11;

        // check that capacity constraints are satisfied
        for v in 0 .. self.graph.v() {
            for e in self.graph.adj(v) {
                let e = e.read().unwrap();
                if e.flow() < -EPSILON || e.flow() > e.capacity() + EPSILON {
                    panic!("edge does not satisfy capacity constraints");
                    // return false;
                }
            }
        }

        // check that net flow into a vertex equals zero, except at source and sink
        if (self.value + self.excess(s)).abs() > EPSILON {
            panic!("Excess at source = {}", self.excess(s));
            // return false;
        }

        for v in 0 .. self.graph.v() {
            if v == s || v == t {
                continue;
            } else {
                if self.excess(v).abs() > EPSILON {
                    panic!("net flow out of {:?} doesn't equal zero", v);
                    // return false;
                }
            }
        }
        return true;
    }
}

impl FlowNetwork {
    pub fn ford_fulkerson<'a>(&'a mut self, s: usize, t: usize) -> FordFulkerson<'a> {
        assert!(s != t, "source equal to sink");
        FordFulkerson::new(self, s, t)
    }
}


#[test]
fn test_flow_network() {
    let mut g = FlowNetwork::new(6);

    g.add_edge(FlowEdge::new(0, 1, 2.0));
    g.add_edge(FlowEdge::new(0, 2, 3.0));
    g.add_edge(FlowEdge::new(1, 3, 3.0));
    g.add_edge(FlowEdge::new(1, 4, 1.0));
    g.add_edge(FlowEdge::new(2, 3, 1.0));
    g.add_edge(FlowEdge::new(2, 4, 1.0));
    g.add_edge(FlowEdge::new(3, 5, 2.0));
    g.add_edge(FlowEdge::new(4, 3, 3.0));

    // println!("=> \n{}", g.to_dot());
    {
        let maxflow = g.ford_fulkerson(0, 5);
        assert_eq!(2.0, maxflow.value());
    }
    // println!("maxflow => \n{}", g.to_dot());
}
