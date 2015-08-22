use std::iter;

pub struct KMP<'a> {
    r: usize,
    dfa: Vec<Vec<usize>>,
    pat: &'a str
}

impl<'a> KMP<'a> {
    // create the DFA from a string pattern
    pub fn new<'b>(pat: &'b str) -> KMP<'b> {
        let r = 256;
        let m = pat.len();
        let dfa = iter::repeat(iter::repeat(0).take(m).collect::<Vec<usize>>())
            .take(r).collect();

        let mut ret = KMP {
            r: r,
            dfa: dfa,
            pat: pat
        };
        ret.build_dfa();
        ret
    }

    fn build_dfa(&mut self) {
        let m = self.pat.len();
        let r = self.r;
        self.dfa[self.pat.char_at(0) as usize][0] = 1;
        let mut x = 0;
        for j in 1 .. m {
            for c in 0 .. r {
                self.dfa[c][j] = self.dfa[c][x]; // copy mismatch cases
            }
            self.dfa[self.pat.char_at(j) as usize][j] = j+1; // set match case
            x = self.dfa[self.pat.char_at(j) as usize][x];
        }
    }

    pub fn search(&self, txt: &str) -> Option<usize> {
        let m = self.pat.len();
        let n = txt.len();
        let mut i = 0;
        let mut j = 0;
        while i < n && j < m {
            j = self.dfa[txt.char_at(i) as usize][j];
            i += 1;
        }
        if j == m {
            Some(i-m)
        } else {
            None
        }
    }
}


#[test]
fn test_kmp() {
    let pat = "abracadabra";
    let text = "abacadabrabracabracadabrabrabracad";
    let kmp = KMP::new(pat);
    assert!(kmp.search(text).map_or(false, |pos| text[pos..].starts_with(pat)));
}
