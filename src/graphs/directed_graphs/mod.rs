use std::iter;
use super::super::fundamentals::stacks_and_queues::bag::Bag;
use super::super::fundamentals::stacks_and_queues::{Stack, Queue};
use super::super::fundamentals::stacks_and_queues::linked_stack;
use super::super::fundamentals::stacks_and_queues::resizing_array_queue::ResizingArrayQueue;

#[derive(Clone, Debug)]
pub struct Digraph {
    v: usize,
    e: usize,
    adj: Vec<Bag<usize>>
}

impl Digraph {
    pub fn new(v: usize) -> Digraph {
        Digraph {
            v: v,
            e: 0,
            adj: iter::repeat(Bag::<usize>::new()).take(v).collect()
        }
    }

    fn validate_vertex(&self, v: usize) {
        assert!(v < self.v, "vertex is not between 0 and {}", self.v - 1)
    }

    pub fn v(&self) -> usize {
        self.v
    }

    pub fn e(&self) -> usize {
        self.e
    }

    pub fn add_edge(&mut self, v: usize, w: usize) {
        self.validate_vertex(v);
        self.validate_vertex(w);

        self.e += 1;
        self.adj[v].add(w);
    }

    pub fn outdegree(&self, v: usize) -> usize {
        self.validate_vertex(v);
        self.adj[v].len()
    }

    pub fn number_of_self_loops(&self) -> usize {
        let mut count = 0;
        for v in 0 .. self.v() {
            for w in self.adj(v) {
                if v == w {
                    count += 1;
                }
            }
        }
        count / 2
    }

    pub fn to_dot(&self) -> String {
        let mut dot = String::new();

        dot.push_str("digraph G {\n");
        for i in 0 .. self.v {
            dot.push_str(&format!("  {};\n", i));
        }

        for (v, adj) in self.adj.iter().enumerate() {
            for w in adj.iter() {
                dot.push_str(&format!("  {} -> {};\n", v, w));
            }
        }
        dot.push_str("}\n");
        dot
    }

    pub fn adj(&self, v: usize) -> Vec<usize> {
        self.adj[v].iter().map(|v| v.clone()).collect::<Vec<usize>>()
    }

    pub fn reverse(&self) -> Digraph {
        let v = self.v;
        let mut adj = iter::repeat(Bag::new()).take(v).collect::<Vec<Bag<usize>>>();
        for s in 0 .. v {
            for e in self.adj(s) {
                adj[e].add(s);
            }
        }
        Digraph {
            v: v,
            e: self.e,
            adj: adj
        }
    }

    pub fn dfs<'a>(&'a self, s: usize) -> SearchPaths<'a> {
        let mut path = SearchPaths::new(self, s);
        path.dfs(s);
        path
    }

    pub fn bfs<'a>(&'a self, s: usize) -> SearchPaths<'a> {
        let mut path = SearchPaths::new(self, s);
        path.bfs(s);
        path
    }

    pub fn reverse_dfs_postorder<'a>(&'a self) -> linked_stack::IntoIter<usize> {
        let mut dfo = DepthFirstOrder::new(self);
        dfo.init();
        dfo.reverse_post.into_iter()
    }

    pub fn kosaraju_sharir_scc<'a>(&'a self) -> KosarajuSharirSCC<'a> {
        KosarajuSharirSCC::new(self)
    }
}

pub struct SearchPaths<'a> {
    graph: &'a Digraph,
    marked: Vec<bool>,
    edge_to: Vec<Option<usize>>,
    s: usize
}

impl<'a> SearchPaths<'a> {
    fn new<'b>(graph: &'b Digraph, s: usize) -> SearchPaths<'b> {
        let marked = iter::repeat(false).take(graph.v()).collect();
        let edge_to = iter::repeat(None).take(graph.v()).collect();
        SearchPaths {
            graph: graph,
            marked: marked,
            edge_to: edge_to,
            s: s
        }
    }

    fn dfs(&mut self, v: usize) {
        self.marked[v] = true;
        for w in self.graph.adj(v) {
            if !self.marked[w] {
                self.dfs(w);
                self.edge_to[w] = Some(v);
            }
        }
    }

    fn bfs(&mut self, s: usize) {
        let mut q = ResizingArrayQueue::new();
        q.enqueue(s);
        self.marked[s] = true;
        while !q.is_empty() {
            let v = q.dequeue().unwrap();
            for w in self.graph.adj(v) {
                if !self.marked[w] {
                    self.edge_to[w] = Some(v);
                    q.enqueue(w);
                    self.marked[w] = true;
                }
            }
        }
    }

    pub fn has_path_to(&self, v: usize) -> bool {
        self.marked[v]
    }

    pub fn path_to(&self, v: usize) -> Option<Vec<usize>> {
        if !self.has_path_to(v) {
            None
        } else {
            let mut path = linked_stack::LinkedStack::new();
            let mut x = v;
            while x != self.s {
                path.push(x);
                x = self.edge_to[x].unwrap();
            }
            path.push(self.s);
            Some(path.into_iter().collect())
        }
    }
}

pub struct DepthFirstOrder<'a> {
    graph: &'a Digraph,
    marked: Vec<bool>,
    reverse_post: linked_stack::LinkedStack<usize>
}

impl<'a> DepthFirstOrder<'a> {
    fn new<'b>(graph: &'b Digraph) -> DepthFirstOrder<'b> {
        let marked = iter::repeat(false).take(graph.v()).collect();
        DepthFirstOrder {
            graph: graph,
            marked: marked,
            reverse_post: Stack::new()
        }
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
        for w in self.graph.adj(v) {
            if !self.marked[w] {
                self.dfs(w);
            }
        }
        self.reverse_post.push(v);
    }
}

/// Compute the strongly-connected components of a digraph using the
/// Kosaraju-Sharir algorithm.
pub struct KosarajuSharirSCC<'a> {
    graph: &'a Digraph,
    marked: Vec<bool>,
    id: Vec<Option<usize>>,
    count: usize,
}

impl<'a> KosarajuSharirSCC<'a> {
    fn new<'b>(graph: &'b Digraph) -> KosarajuSharirSCC<'b> {
        let n = graph.v();
        let mut cc = KosarajuSharirSCC {
            graph: graph,
            marked: iter::repeat(false).take(n).collect(),
            id: iter::repeat(None).take(n).collect(),
            count: 0
        };
        cc.init();
        cc
    }

    fn init(&mut self) {
        for v in self.graph.reverse().reverse_dfs_postorder() {
            if !self.marked[v] {
                self.dfs(v);
                self.count += 1;
            }
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn id(&self, v: usize) -> usize {
        self.id[v].unwrap()
    }

    pub fn connected(&self, v: usize, w: usize) -> bool {
        self.id[v] == self.id[w]
    }

    fn dfs(&mut self, v: usize) {
        self.marked[v] = true;
        self.id[v] = Some(self.count);
        for w in self.graph.adj(v) {
            if !self.marked[w] {
                self.dfs(w)
            }
        }
    }
}


#[test]
fn test_digraph_visit() {
    let mut g = Digraph::new(13);
    g.add_edge(0, 1);
    g.add_edge(2, 0);
    g.add_edge(6, 0);
    g.add_edge(0, 5);

    g.add_edge(3, 5);
    g.add_edge(5, 4);

    g.add_edge(2, 3);
    g.add_edge(3, 2);

    g.add_edge(4, 3);
    g.add_edge(4, 2);
    g.add_edge(6, 4);

    g.add_edge(6, 8);
    g.add_edge(8, 6);

    g.add_edge(7, 6);

    g.add_edge(7, 9);
    g.add_edge(6, 9);

    g.add_edge(9, 10);
    g.add_edge(10, 12);
    g.add_edge(9, 11);
    g.add_edge(12, 9);
    g.add_edge(11, 12);

    // println!("dot => \n {}", g.to_dot());
    // println!("dot => \n {}", g.reverse().to_dot());
    assert_eq!(format!("{:?}", g.dfs(0).path_to(3).unwrap()), "[0, 5, 4, 2, 3]");
    assert_eq!(format!("{:?}", g.bfs(0).path_to(3).unwrap()), "[0, 5, 4, 3]");

    let scc = g.kosaraju_sharir_scc();

    assert!(scc.connected(6, 8));
    assert!(scc.connected(9, 12));
    assert!(!scc.connected(6, 7));
}


#[test]
fn test_digraph() {
    let mut g = Digraph::new(10);
    g.add_edge(0, 3);
    g.add_edge(0, 5);
    g.add_edge(4, 5);
    g.add_edge(2, 9);
    g.add_edge(2, 8);
    g.add_edge(3, 7);

    g.add_edge(1, 6);
    g.add_edge(6, 9);
    g.add_edge(5, 8);

    // println!("got => \n{}", g.to_dot());

    assert_eq!(10, g.v());
    assert_eq!(9, g.e());
    assert_eq!(1, g.outdegree(5));

    for w in g.adj(5) {
        assert!(vec![8, 4, 0].contains(&w));
    }

    assert_eq!(g.number_of_self_loops(), 0);
}

#[test]
fn test_digraph_functions() {
    let mut g = Digraph::new(5);
    for i in 0 .. 5 {
        for j in 0 .. 5 {
            g.add_edge(i, j);
        }
    }

    assert_eq!(2, g.number_of_self_loops());
}

#[test]
fn test_digraph_depth_first_search_order() {
    let mut g = Digraph::new(7);
    g.add_edge(0, 2);
    g.add_edge(0, 5);
    g.add_edge(0, 1);
    g.add_edge(6, 0);
    g.add_edge(5, 2);
    g.add_edge(3, 2);
    g.add_edge(3, 5);
    g.add_edge(1, 4);
    g.add_edge(3, 4);
    g.add_edge(3, 6);
    g.add_edge(6, 4);

    let dfo: Vec<usize> = g.reverse_dfs_postorder().collect();
    assert_eq!(vec![3, 6, 0, 5, 2, 1, 4], dfo);
}
