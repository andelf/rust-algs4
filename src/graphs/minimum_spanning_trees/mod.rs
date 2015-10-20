use std::fmt;
use std::iter;
use std::cmp;
use std::f64;

use adivon::priority_queue::IndexMinPQ;
use adivon::{Bag, Queue, MinPQ};
use adivon::UnionFind;

/// a weighted edge
#[derive(Clone, Copy)]
pub struct Edge {
    v: usize,
    w: usize,
    weight: f64
}

impl Edge {
    pub fn new(v: usize, w: usize, weight: f64) -> Edge {
        Edge { v: v, w: w, weight: weight }
    }

    pub fn weight(&self) -> f64 {
        self.weight
    }

    pub fn either(&self) -> usize {
        self.v
    }

    pub fn other(&self, vertex: usize) -> usize {
        if vertex == self.v {
            self.w
        } else if vertex == self.w {
            self.v
        } else {
            panic!("illegal endppint")
        }
    }

    fn swap(mut self) -> Edge {
        let v = self.v;
        self.v = self.w;
        self.w =v;
        self
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        self.weight == other.weight
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Edge) -> Option<cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{} {:.5}", self.v, self.w, self.weight)
    }
}

impl fmt::Debug for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}({:.2})", self.v, self.w, self.weight)
    }
}


pub struct EdgeWeightedGraph {
    v: usize,
    e: usize,
    adj: Vec<Bag<Edge>>
}

/// an edge-weighted graph
impl EdgeWeightedGraph {
    pub fn new(v: usize) -> EdgeWeightedGraph {
        EdgeWeightedGraph {
            v: v,
            e: 0,
            adj: iter::repeat(Bag::new()).take(v).collect::<Vec<Bag<Edge>>>()
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
        assert!(v < self.v, "vertex is not between 0 and max V")
    }

    pub fn add_edge(&mut self, e: Edge) {
        let v = e.either();
        let w = e.other(v);
        self.validate_vertex(v);
        self.validate_vertex(w);
        self.adj[v].add(e.clone());
        if v != w {
            self.adj[w].add(e.swap());
        }
        self.e += 1
    }

    // this implements IntoIterator
    pub fn adj(&self, v: usize) -> Vec<Edge> {
        self.validate_vertex(v);
        self.adj[v].iter().map(|e| e.clone()).collect::<Vec<Edge>>()
    }

    pub fn degree(&self, v: usize) -> usize {
        self.validate_vertex(v);
        self.adj[v].len()
    }

    pub fn edges(&self) -> Vec<Edge> {
        self.adj
            .iter()
            .flat_map(|adj| {
                adj.iter().map(|e| e.clone()).collect::<Vec<Edge>>().into_iter()
            })
            .filter(|ref e| e.either() <= e.other(e.either()) )
            .collect::<Vec<Edge>>()
    }

    pub fn to_dot(&self) -> String {
        let mut dot = String::new();

        dot.push_str("graph G {\n");
        for i in 0 .. self.v {
            dot.push_str(&format!("  {};\n", i));
        }

        for e in self.edges() {
            let v = e.either();
            let w = e.other(v);
            dot.push_str(&format!("  {} -- {} [ label=\"{}\" ];\n",
                                  v, w, e.weight))
        }
        dot.push_str("}\n");
        dot
    }
}


#[test]
fn test_edge_weighted_graph() {
    let mut g = EdgeWeightedGraph::new(6);
    g.add_edge(Edge::new(0, 1, 7.0));
    g.add_edge(Edge::new(1, 2, 10.0));
    g.add_edge(Edge::new(0, 2, 9.0));
    g.add_edge(Edge::new(0, 5, 14.0));
    g.add_edge(Edge::new(1, 3, 15.0));
    g.add_edge(Edge::new(2, 5, 2.0));
    g.add_edge(Edge::new(2, 3, 11.0));
    g.add_edge(Edge::new(4, 5, 9.0));
    g.add_edge(Edge::new(3, 4, 6.0));
    g.add_edge(Edge::new(2, 2, 1.0));

    assert_eq!(10, g.edges().len());
    assert!(!g.to_dot().is_empty());
    // println!("got => \n{}", g.to_dot());
}


#[allow(dead_code)]
pub struct KruskalMST<'a> {
    graph: &'a EdgeWeightedGraph,
    weight: f64,
    mst: Queue<Edge>
}

impl<'a> KruskalMST<'a> {
    fn new<'b>(graph: &'b EdgeWeightedGraph) -> KruskalMST<'b> {
        let n = graph.v();
        let mut weight = 0f64;
        let mut mst = Queue::<Edge>::new();
        let mut pq = MinPQ::<Edge>::new();
        for e in graph.edges() {
            pq.insert(e);
        }
        let mut uf = UnionFind::new(n);

        while !pq.is_empty() && mst.len() < n - 1 {
            let e = pq.del_min().unwrap();
            let v = e.either();
            let w = e.other(v);
            if !uf.connected(v, w) {
                uf.union(v, w);
                weight += e.weight();
                mst.enqueue(e);
            }
        }

        KruskalMST {
            graph: graph,
            weight: weight,
            mst: mst
        }
    }

    pub fn edges(&self) -> Vec<Edge> {
        self.mst.clone().into_iter().collect::<Vec<Edge>>()
    }
}

impl EdgeWeightedGraph {
    pub fn kruskal_mst<'a>(&'a self) -> KruskalMST<'a> {
        KruskalMST::new(self)
    }
}

/// data type for computing a minimum spanning tree in an edge-weighted graph
pub struct LazyPrimMST<'a> {
    graph: &'a EdgeWeightedGraph,
    weight: f64,
    mst: Queue<Edge>,
    marked: Vec<bool>,
    pq: MinPQ<Edge>
}

impl<'a> LazyPrimMST<'a> {
    fn new<'b>(graph: &'b EdgeWeightedGraph) -> LazyPrimMST<'b> {
        let n = graph.v();
        let marked = iter::repeat(false).take(n).collect();
        let pq = MinPQ::new();
        let mst = Queue::new();

        let mut ret = LazyPrimMST {
            graph: graph,
            weight: 0.0,
            mst: mst,
            marked: marked,
            pq: pq
        };
        // run Prim for all vertices
        // get a minimum spanning forest
        for v in 0 .. n {
            if !ret.marked[v] {
                ret.prim(v);
            }
        }
        ret
    }

    // Prim's algorithm
    fn prim(&mut self, s: usize) {
        self.visit(s);

        while !self.pq.is_empty() {
            let e = self.pq.del_min().unwrap();
            let v = e.either();
            let w = e.other(v);

            if self.marked[v] && self.marked[w] {
                continue;
            }
            self.weight += e.weight();
            self.mst.enqueue(e);
            if !self.marked[v] {
                self.visit(v);
            }
            if !self.marked[w] {
                self.visit(w)
            }
        }
    }

    fn visit(&mut self, v: usize) {
        self.marked[v] = true;
        for e in self.graph.adj(v) {
            if !self.marked[e.other(v)] {
                self.pq.insert(e)
            }
        }
    }

    pub fn edges(&self) -> Vec<Edge> {
        self.mst.clone().into_iter().collect::<Vec<Edge>>()
    }
}

impl EdgeWeightedGraph {
     pub fn lazy_prim_mst<'a>(&'a self) -> LazyPrimMST<'a> {
         LazyPrimMST::new(self)
    }
}

pub struct PrimMST<'a> {
    graph: &'a EdgeWeightedGraph,
    edge_to: Vec<Option<Edge>>,
    dist_to: Vec<f64>,
    marked: Vec<bool>,
    pq: IndexMinPQ<f64>
}

impl<'a> PrimMST<'a> {
    fn new<'b>(graph: &'b EdgeWeightedGraph) -> PrimMST<'b> {
        let n = graph.v();
        let edge_to = iter::repeat(None).take(n).collect();
        let dist_to = iter::repeat(f64::INFINITY).take(n).collect();
        let marked = iter::repeat(false).take(n).collect();
        let pq = IndexMinPQ::with_capacity(n);

        let mut ret = PrimMST {
            graph: graph,
            edge_to: edge_to,
            dist_to: dist_to,
            marked: marked,
            pq: pq
        };
        // run Prim for all vertices
        // get a minimum spanning forest
        for v in 0 .. n {
            if !ret.marked[v] {
                ret.prim(v);
            }
        }
        ret
    }

    // Prim's algorithm, start from vertex s
    fn prim(&mut self, s: usize) {
        self.dist_to[s] = 0.0;
        self.pq.insert(s, 0.0);

        while !self.pq.is_empty() {
            let v = self.pq.del_min().unwrap();
            self.scan(v);
        }
    }

    fn scan(&mut self, v: usize) {
        self.marked[v] = true;
        for e in self.graph.adj(v) {
            let w = e.other(v);
            if self.marked[w] {
                continue;
            }
            if e.weight() < self.dist_to[w] {
                self.dist_to[w] = e.weight();
                self.edge_to[w] = Some(e);
                if self.pq.contains(w) {
                    self.pq.decrease_key(w, self.dist_to[w]);
                } else {
                    self.pq.insert(w, self.dist_to[w]);
                }
            }
        }
    }

    pub fn edges(&self) -> Vec<Edge> {
        let mut mst = Queue::new();
        for e in self.edge_to.iter() {
            e.map(|e| mst.enqueue(e.clone()));
        }
        mst.into_iter().collect::<Vec<Edge>>()
    }
}

impl EdgeWeightedGraph {
     pub fn prim_mst<'a>(&'a self) -> PrimMST<'a> {
         PrimMST::new(self)
    }
}


#[test]
fn test_edge_weighted_graph_mst() {
    let mut g = EdgeWeightedGraph::new(6);
    g.add_edge(Edge::new(0, 1, 7.0));
    g.add_edge(Edge::new(1, 2, 10.0));
    g.add_edge(Edge::new(0, 2, 9.0));
    g.add_edge(Edge::new(0, 5, 14.0));
    g.add_edge(Edge::new(1, 3, 15.0));
    g.add_edge(Edge::new(2, 5, 2.0));
    g.add_edge(Edge::new(2, 3, 11.0));
    g.add_edge(Edge::new(4, 5, 9.0));
    g.add_edge(Edge::new(3, 4, 6.0));
    g.add_edge(Edge::new(2, 2, 1.0));

    assert_eq!(33.0, g.kruskal_mst().weight);
    assert_eq!(33.0, g.kruskal_mst().edges().iter().map(|e| e.weight).sum());

    assert_eq!(33.0, g.lazy_prim_mst().weight);
    assert_eq!(33.0, g.lazy_prim_mst().edges().iter().map(|e| e.weight).sum());

    assert_eq!(33.0, g.prim_mst().edges().iter().map(|e| e.weight).sum());
}
