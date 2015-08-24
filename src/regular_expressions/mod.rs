use adivon::{Stack, Digraph};

pub struct NFA {
    graph: Digraph,
    re: Vec<char>,
    m: usize
}

impl NFA {
    pub fn new(regexp: &str) -> NFA {
        NFA {
            graph: NFA::build_epsilon_transition_digraph(regexp),
            re: regexp.chars().collect(),
            m: regexp.len()
        }
    }

    fn build_epsilon_transition_digraph(regexp: &str) -> Digraph {
        let m = regexp.len();
        let mut ops = Stack::<usize>::new();
        let mut g = Digraph::new(m+1);

        for i in 0 .. m {
            let mut lp = i;
            if regexp.char_at(i) == '(' || regexp.char_at(i) == '|' {
                ops.push(i);
            } else if regexp.char_at(i) == ')' {
                let or = ops.pop().unwrap();

                // 2-way or operator
                if regexp.char_at(or) == '|' {
                    lp = ops.pop().unwrap();
                    g.add_edge(lp, or+1);
                    g.add_edge(or, i);
                } else if regexp.char_at(or) == '(' {
                    lp == or;
                } else {
                    assert!(false, "bad regexp format");
                }
            }

            // closure operator (use 1-char lookahead)
            if i < m-1 && regexp.char_at(i+1) == '*' {
                g.add_edge(lp, i+1);
                g.add_edge(i+1, lp);
            }
            if regexp.char_at(i) == '(' || regexp.char_at(i) == '*' || regexp.char_at(i) == ')' {
                g.add_edge(i, i+1);
            }
        }
        g
    }

    pub fn recognize(&self, txt: &str) -> bool {
        let mut dfs = self.graph.dfs(0);
        // use Vec as Bag
        let mut pc = Vec::with_capacity(self.m);
        let regexp = &self.re;
        for v in 0 .. self.graph.v() {
            if dfs.has_path_to(v) {
                pc.push(v);
            }
        }

        // Compute possible NFA states for txt[i+1]
        for i in 0 .. txt.len() {
            // use Vec as Bag
            let mut matches = Vec::new();
            for v in pc.iter().map(Clone::clone) {
                if v == self.m { continue }
                if regexp[v] == txt.char_at(i) || regexp[v] == '.' {
                    matches.push(v+1);
                }
            }

            dfs = self.graph.dfs_multi_source(matches);
            pc = Vec::with_capacity(self.m);
            for v in 0 .. self.graph.v() {
                if dfs.has_path_to(v) {
                    pc.push(v);
                }
            }

            // optimization if no states
            if pc.len() == 0 {
                return false
            }
        }

        for v in pc {
            if v == self.m {
                return true;
            }
        }
        return false;
    }
}


#[test]
fn test_regexp() {
    // must be wrapped in ()
    let pattern = NFA::new("((A*B|AC)D)");
    assert!(pattern.recognize("AABD"));
    assert!(pattern.recognize("ACD"));
    assert!(!pattern.recognize("ABCD"));
}
