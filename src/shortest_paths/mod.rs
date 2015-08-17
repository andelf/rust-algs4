use std::fmt;
use std::iter;
use std::f64;

use adivon::bag::Bag;
use adivon::stack::Stack;
use adivon::queue::Queue;
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

    // FIXME: should this be public?
    pub fn check(&self) -> bool {
        let s = self.s;
        for e in self.graph.edges() {
            if e.weight() < 0.0 {
                return false;
            }
        }

        if self.dist_to[s] != 0.0 || self.edge_to[s].is_some() {
            return false;
        }

        for v in 0 .. self.graph.v() {
            if v == s { continue }
            if self.edge_to[v].is_none() && self.dist_to[v] != f64::INFINITY {
                // dist_to[] edge_to[] inconsistent
                return false;
            }
        }

        for v in 0 .. self.graph.v() {
            for e in self.graph.adj(v) {
                let w  = e.to();
                if self.dist_to[v] + e.weight() < self.dist_to[w] {
                    // edge not relaxed
                    return false;
                }
            }
        }

        for w in 0 .. self.graph.v() {
            if self.edge_to[w].is_none() { continue }
            let e = self.edge_to[w].unwrap();
            let v = e.from();
            if w != e.to() {
                return false;
            }
            if self.dist_to[v] + e.weight() != self.dist_to[w] {
                // edge on shortest path not tight
                return false;
            }
        }

        true
    }
}


impl EdgeWeightedDigraph {
    pub fn dijkstra_sp<'a>(&'a self, s: usize) -> DijkstraSP<'a> {
        DijkstraSP::new(self, s)
    }
}

/// Compute preorder and postorder for a digraph or edge-weighted digraph
pub struct DepthFirstOrder<'a> {
    graph: &'a EdgeWeightedDigraph,
    pre: Vec<usize>,
    post: Vec<usize>,
    preorder: Queue<usize>,
    postorder: Queue<usize>,
    marked: Vec<bool>,
    pre_counter: usize,
    post_counter: usize
}

impl<'a> DepthFirstOrder<'a> {
    fn new<'b>(graph: &'b EdgeWeightedDigraph) -> DepthFirstOrder<'b> {
        let n = graph.v();
        let mut ret = DepthFirstOrder {
            graph: graph,
            pre: iter::repeat(0).take(n).collect(),
            post: iter::repeat(0).take(n).collect(),
            preorder: Queue::new(),
            postorder: Queue::new(),
            marked: iter::repeat(false).take(n).collect(),
            pre_counter: 0,
            post_counter: 0
        };
        ret.init();
        ret
    }

    fn init(&mut self) {
        for v in 0 .. self.graph.v() {
            if !self.marked[v] {
                self.dfs(v)
            }
        }
    }

    fn dfs(&mut self, v: usize) {
        self.marked[v] = true;
        self.pre[v] = self.pre_counter;
        self.pre_counter += 1;
        self.preorder.enqueue(v);
        for e in self.graph.adj(v) {
            let w = e.to();
            if !self.marked[w] {
                self.dfs(w);
            }
        }
        self.postorder.enqueue(v);
        self.post[v] = self.post_counter;
        self.post_counter += 1;
    }

    // preorder number of vertex v
    pub fn preorder(&self, v: usize) -> usize {
        self.pre[v]
    }

    // postorder number of vertex v
    pub fn postorder(&self, v: usize) -> usize {
        self.post[v]
    }

    pub fn pre(&self) -> ::std::vec::IntoIter<usize> {
        self.preorder.clone().into_iter().collect::<Vec<usize>>().into_iter()
    }

    pub fn post(&self) -> ::std::vec::IntoIter<usize> {
        self.postorder.clone().into_iter().collect::<Vec<usize>>().into_iter()
    }

    pub fn reverse_post(&self) -> ::std::vec::IntoIter<usize> {
        let mut reverse = Stack::new();
        for v in self.postorder.iter() {
            reverse.push(*v);
        }
        reverse.into_iter().collect::<Vec<usize>>().into_iter()
    }

    fn check(&self) -> bool {
        let mut r = 0;
        for v in self.post() {
            if self.postorder(v) != r {
                // post(v) and post() inconsistent
                return false;
            }
            r += 1;
        }

        r = 0;
        for v in self.pre() {
            if self.preorder(v) != r {
                // preorder(v) and pre() inconsistent
                return false;
            }
            r += 1;
        }
        return true;
    }
}


impl EdgeWeightedDigraph {
    pub fn depth_first_order<'a>(&'a self) -> DepthFirstOrder<'a> {
        DepthFirstOrder::new(self)
    }
}

// Finds a directed cycle in an edge-weighted digraph
pub struct EdgeWeightedDirectedCycle<'a> {
    graph: &'a EdgeWeightedDigraph,
    marked: Vec<bool>,
    edge_to: Vec<Option<DirectedEdge>>,
    on_stack: Vec<bool>,
    // directed cycle (or empty)
    cycle: Option<Stack<DirectedEdge>>
}

impl<'a> EdgeWeightedDirectedCycle<'a> {
    fn new<'b>(graph: &'b EdgeWeightedDigraph) -> EdgeWeightedDirectedCycle<'b> {
        let n = graph.v();
        let mut ret = EdgeWeightedDirectedCycle {
            graph: graph,
            marked: iter::repeat(false).take(n).collect(),
            edge_to: iter::repeat(None).take(n).collect(),
            on_stack: iter::repeat(false).take(n).collect(),
            cycle: None
        };
        ret.init();
        ret
    }

    fn init(&mut self) {
        for v in 0 .. self.graph.v() {
            if !self.marked[v] {
                self.dfs(v)
            }
        }
    }

    fn dfs(&mut self, v: usize) {
        self.on_stack[v] = true;
        self.marked[v] = true;
        for e in self.graph.adj(v) {
            let w = e.to();

            if self.cycle.is_some() {
                return;
            } else if !self.marked[w] {
                self.edge_to[w] = Some(e);
                self.dfs(w);
            } else if self.on_stack[w] {
                self.cycle = Some(Stack::new());
                // scope local
                let mut e = e.clone();
                while e.from() != w {
                    self.cycle.as_mut().map(|s| s.push(e));
                    e = self.edge_to[e.from()].unwrap();
                }
                self.cycle.as_mut().map(|s| s.push(e));
            }
        }
        self.on_stack[v] = false;
    }

    pub fn has_cycle(&self) -> bool {
        self.cycle.is_some()
    }

    pub fn cycle(&self) -> ::std::vec::IntoIter<DirectedEdge> {
        self.cycle.iter().flat_map(|e| e.clone()).collect::<Vec<DirectedEdge>>().into_iter()
    }

    pub fn check(&self) -> bool {
        if self.has_cycle() {
            let first = self.cycle().next().unwrap();
            let last = self.cycle().last().unwrap();

            if first.from() == last.to() {
                return true;
            } else {
                return false;
            }
        }
        return true;
    }
}


impl EdgeWeightedDigraph {
    pub fn cycle(&self) -> ::std::vec::IntoIter<DirectedEdge> {
        EdgeWeightedDirectedCycle::new(self).cycle()
    }

    pub fn has_cycle(&self) -> bool {
        EdgeWeightedDirectedCycle::new(self).has_cycle()
    }
}

/// Compute topological ordering of a DAG or edge-weighted DAG
pub struct Topological {
    order: Vec<usize>
}

impl Topological {
    pub fn new(graph: EdgeWeightedDigraph) -> Topological {
        unimplemented!()
    }
}

// Computes shortest paths in an edge-weighted acyclic digraph
pub struct AcyclicSP<'a> {
    graph: &'a EdgeWeightedDigraph,
    dist_to: Vec<f64>,
    edge_to: Vec<Option<DirectedEdge>>
}

impl<'a> AcyclicSP<'a> {
    fn new<'b>(graph: &'b EdgeWeightedDigraph, s: usize) -> AcyclicSP<'b> {
        let n = graph.v();
        let mut dist_to: Vec<f64> = iter::repeat(f64::INFINITY).take(n).collect();
        let edge_to = iter::repeat(None).take(n).collect();

        dist_to[s] = 0.0;
        AcyclicSP {
            graph: graph,
            dist_to: dist_to,
            edge_to: edge_to
        }
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

    assert!(g.dijkstra_sp(0).check());
}


#[test]
fn test_cyclic_edge_weighted_directed_graph() {
    let mut g = EdgeWeightedDigraph::new(4);
    g.add_edge(DirectedEdge::new(0, 1, 0.5));
    g.add_edge(DirectedEdge::new(0, 2, 0.5));
    g.add_edge(DirectedEdge::new(1, 2, 0.5));
    g.add_edge(DirectedEdge::new(2, 3, 0.5));
    g.add_edge(DirectedEdge::new(3, 1, 0.5));

    assert!(g.has_cycle());
    assert_eq!(3, g.cycle().count());
}



#[test]
fn test_acyclic_shortest_path() {
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


    assert!(g.depth_first_order().check());
}
