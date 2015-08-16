use std::fmt;
use std::iter;
use std::f64;

use adivon::bag::Bag;
use adivon::stack::Stack;
use adivon::priority_queue::IndexMinPQ;

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


// Single-source shortest paths API
pub struct DijkstraSP<'a> {
    graph: &'a EdgeWeightedDigraph,
    dist_to: Vec<f64>,
    edge_to: Vec<Option<DirectedEdge>>,
    pq: IndexMinPQ<f64>,
    s: usize
}

impl<'a> DijkstraSP<'a> {
    fn new<'b>(graph: &'b EdgeWeightedDigraph, s: usize) -> DijkstraSP<'b> {
        let n = graph.v();
        for e in graph.edges() {
            if e.weight() < 0.0 {
                panic!("edge has negative weight in DijkstraSP");
            }
        }
        let dist_to = iter::repeat(f64::INFINITY).take(n).collect();
        let edge_to = iter::repeat(None).take(n).collect();
        let pq = IndexMinPQ::with_capacity(n);
        let mut sp = DijkstraSP {
            graph: graph,
            s: s,
            dist_to: dist_to,
            edge_to: edge_to,
            pq: pq
        };
        // alogrithm
        sp.dist_to[s] = 0.0;

        sp.pq.insert(s, 0.0);
        while !sp.pq.is_empty() {
            let v = sp.pq.del_min().unwrap();
            for e in graph.adj(v) {
                sp.relax(e);
            }
        }

        sp
    }

    fn relax(&mut self, e: DirectedEdge) {
        let v = e.from();
        let w = e.to();
        if self.dist_to[w] > self.dist_to[v] + e.weight() {
            self.dist_to[w] = self.dist_to[v] + e.weight();
            self.edge_to[w] = Some(e);
            if self.pq.contains(w) {
                self.pq.decrease_key(w, self.dist_to[w]);
            } else {
                self.pq.insert(w, self.dist_to[w]);
            }
        }
    }

    // length of shortest path from s to v
    pub fn dist_to(&self, v: usize) -> f64 {
        self.dist_to[v]
    }

    pub fn has_path_to(&self, v: usize) -> bool {
        self.dist_to[v] < f64::INFINITY
    }

    // shortest path from s to v
    pub fn path_to(&self, v: usize) -> ::std::vec::IntoIter<DirectedEdge> {
        if !self.has_path_to(v) {
            vec!().into_iter()
        } else {
            let mut path = Stack::new();
            let mut e = self.edge_to[v];
            while e.is_some() {
                path.push(e.unwrap());
                e = self.edge_to[e.unwrap().from()]
            }
            path.into_iter().collect::<Vec<DirectedEdge>>().into_iter()
        }
    }
}


impl EdgeWeightedDigraph {
    pub fn dijkstra_sp<'a>(&'a self, s: usize) -> DijkstraSP<'a> {
        DijkstraSP::new(self, s)
    }
}

#[test]
fn test_dijkstra_shortest_path() {
    let mut g = EdgeWeightedDigraph::new(6);
    g.add_edge(DirectedEdge::new(0, 1, 7.0));
    g.add_edge(DirectedEdge::new(1, 2, 10.0));
    g.add_edge(DirectedEdge::new(0, 2, 9.0));
    g.add_edge(DirectedEdge::new(0, 5, 14.0));
    g.add_edge(DirectedEdge::new(1, 3, 15.0));
    g.add_edge(DirectedEdge::new(2, 5, 2.0));
    g.add_edge(DirectedEdge::new(2, 3, 11.0));
    g.add_edge(DirectedEdge::new(4, 5, 9.0));
    g.add_edge(DirectedEdge::new(3, 4, 6.0));
    g.add_edge(DirectedEdge::new(2, 2, 1.0));

    assert_eq!(20.0, g.dijkstra_sp(0).dist_to(3));
    assert_eq!(26.0, g.dijkstra_sp(0).path_to(4).map(|e| e.weight()).sum());
}
