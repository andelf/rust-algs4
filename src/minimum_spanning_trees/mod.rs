use std::fmt;
use std::iter;
use std::cmp;
use super::stacks_and_queues::bag::Bag;
use super::stacks_and_queues::Queue;
use super::stacks_and_queues::resizing_array_queue::ResizingArrayQueue;
use super::priority_queues::MinPQ;
use super::priority_queues::binary_heaps::BinaryHeapMinPQ;
use super::union_find::UF;
use super::union_find::weighted_quick_union::UnionFind;

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

    pub fn adj(&self, v: usize) -> ::std::vec::IntoIter<Edge> {
        self.validate_vertex(v);
        self.adj[v].iter().map(|e| e.clone()).collect::<Vec<Edge>>().into_iter()
    }

    pub fn degree(&self, v: usize) -> usize {
        self.validate_vertex(v);
        self.adj[v].len()
    }

    pub fn edges(&self) -> ::std::vec::IntoIter<Edge> {
        self.adj
            .iter()
            .flat_map(|adj| {
                adj.iter().map(|e| e.clone()).collect::<Vec<Edge>>().into_iter()
            })
            .filter(|ref e| e.either() <= e.other(e.either()) )
            .collect::<Vec<Edge>>()
            .into_iter()
    }

    // use fdp or neato
    pub fn to_dot(&self) -> String {
        let mut dot = String::new();

        dot.push_str("graph G {\n");
        for i in 0 .. self.v {
            dot.push_str(&format!("  {};\n", i));
        }

        for e in self.edges() {
            let v = e.either();
            let w = e.other(v);
            dot.push_str(&format!("  {} -- {} [len={}];\n",
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

    assert_eq!(10, g.edges().count());
    assert!(!g.to_dot().is_empty());
}


#[allow(dead_code)]
pub struct KruskalMST<'a> {
    graph: &'a EdgeWeightedGraph,
    weight: f64,
    mst: ResizingArrayQueue<Edge>
}

impl<'a> KruskalMST<'a> {
    fn new<'b>(graph: &'b EdgeWeightedGraph) -> KruskalMST<'b> {
        let n = graph.v();
        let mut weight = 0f64;
        let mut mst = ResizingArrayQueue::<Edge>::new();
        let mut pq = BinaryHeapMinPQ::<Edge>::new();
        for e in graph.edges() {
            pq.insert(e);
        }
        let mut uf = UnionFind::new(n);

        while !pq.is_empty() && mst.size() < n - 1 {
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

    pub fn edges(&self) -> ::std::vec::IntoIter<Edge> {
        self.mst.clone().into_iter().collect::<Vec<Edge>>().into_iter()
    }
}

impl EdgeWeightedGraph {
    pub fn kruskal_mst<'a>(&'a self) -> KruskalMST<'a> {
        KruskalMST::new(self)
    }
}

#[test]
fn test_edge_weighted_graph_kruskal_mst() {
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
    assert_eq!(33.0, g.kruskal_mst().edges().map(|e| e.weight).sum());

}


/// data type for computing a minimum spanning tree in an edge-weighted graph
pub struct LazyPrimMST<'a> {
    graph: &'a EdgeWeightedGraph,
    weight: f64,
    mst: ResizingArrayQueue<Edge>,
    marked: Vec<bool>,
    pq: BinaryHeapMinPQ<Edge>
}
