use std::iter;

// FIXME: only supports extended-ASCII
/// searches for the pattern in the input text using the
/// KMP algorithm.
pub struct KMP<'a> {
    r: usize,
    dfa: Vec<Vec<usize>>,
    pat: &'a str
}

impl<'a> KMP<'a> {
    // create the DFA from a string pattern
    pub fn new<'b>(pat: &'b str) -> KMP<'b> {
        let r = 256;
        let mut ret = KMP {
            r: r,
            dfa: Vec::new(),
            pat: pat
        };
        ret.build_dfa();
        ret
    }

    fn build_dfa(&mut self) {
        let m = self.pat.len();
        let r = self.r;
        let mut dfa = iter::repeat(iter::repeat(0).take(m).collect::<Vec<usize>>())
            .take(r).collect::<Vec<Vec<usize>>>();
        dfa[self.pat.char_at(0) as usize][0] = 1;
        let mut x = 0;
        for j in 1 .. m {
            for c in 0 .. r {
                dfa[c][j] = dfa[c][x]; // copy mismatch cases
            }
            dfa[self.pat.char_at(j) as usize][j] = j+1; // set match case
            x = dfa[self.pat.char_at(j) as usize][x];   // update restart state
        }
        self.dfa = dfa;
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
    assert_eq!(kmp.search("zzzzz"), None);
}


/// searches for the pattern in the input text using the
/// bad-character rule part of the Boyer-Moore algorithm.
pub struct BoyerMoore<'a> {
    r: usize,
    right: Vec<isize>,
    pat: &'a str
}

impl<'a> BoyerMoore<'a> {
     pub fn new<'b>(pat: &'b str) -> BoyerMoore<'b> {
         let r = 256;
         let mut ret = BoyerMoore {
             r: r,
             right: Vec::new(),
             pat: pat
         };
         ret.init();
         ret
     }

    fn init(&mut self) {
        let r = self.r;
        self.right = iter::repeat(-1).take(r).collect();
        for j in 0 .. self.pat.len() {
            self.right[self.pat.char_at(j) as usize] = j as isize;
        }
    }

    pub fn search(&self, txt: &str) -> Option<usize> {
        let m = self.pat.len();
        let n = txt.len();
        if n < m {
            return None;
        }
        let mut skip;
        let mut i = 0;
        while i <= n-m {
            skip = 0;
            for j in (0 .. m).rev() {
                if self.pat.char_at(j) != txt.char_at(i+j) {
                    skip = j as isize - self.right[txt.char_at(i+j) as usize];
                    if skip < 1 {
                        skip = 1;
                    }
                    break;
                }
            }
            if skip == 0 {
                return Some(i);
            }
            i += skip as usize;
        }
        return None;
    }
}

#[test]
fn test_boyer_moore() {
    let pat = "abracadabra";
    let text = "abacadabrabracabracadabrabrabracad";
    let bm = BoyerMoore::new(pat);
    assert!(bm.search(text).map_or(false, |pos| text[pos..].starts_with(pat)));
    assert_eq!(bm.search("zzzzz"), None);
}

pub struct RabinKarp<'a> {
    pat: &'a str,
    pat_hash: usize,
    m: usize,
    r: usize,
    q: usize,
    rm: usize,                    // R^(M-1) % Q
}

impl<'a> RabinKarp<'a> {
    pub fn new<'b>(pat: &'b str) -> RabinKarp<'b> {
        let mut ret = RabinKarp {
            pat: pat,
            pat_hash: 0,
            m: pat.len(),
            r: 256,
            q: RabinKarp::long_random_prime(),
            rm: 1
        };
        ret.init();
        ret
    }

    fn init(&mut self) {
        let r = self.r;
        let q = self.q;
        for _ in 1 .. self.m {
            self.rm = (r * self.rm) % q;
        }
        self.pat_hash = self.hash(self.pat, self.m);
    }

    fn hash(&self, key: &str, m: usize) -> usize {
        let r = self.r;
        let q = self.q;
        let mut h = 0;
        for j in 0 .. m {
            h = (r * h + key.char_at(j) as usize) % q
        }
        h
    }

    fn check(&self, txt: &str, i: usize) -> bool {
        for j in 0 .. self.m {
            if self.pat.char_at(j) != txt.char_at(i+j) {
                return false;
            }
        }
        true
    }

    pub fn search(&self, txt: &str) -> Option<usize> {
        let n = txt.len();
        if n < self.m {
            return None;
        }

        let mut txt_hash = self.hash(txt, self.m);

        // check for match at offset 0
        if self.pat_hash == txt_hash && self.check(txt, 0) {
            return Some(0);
        }

        // check for hash match
        for i in self.m .. n {
            txt_hash = (txt_hash + self.q - self.rm * txt.char_at(i-self.m) as usize % self.q) % self.q;
            txt_hash = (txt_hash * self.r + txt.char_at(i) as usize) % self.q;

            let offset = i - self.m + 1;
            if self.pat_hash == txt_hash && self.check(txt, offset) {
                return Some(offset);
            }
        }
        None
    }

    pub fn search_monte_carlo(&self, txt: &str) -> Option<usize> {
        if txt.len() < self.m {
            return None;
        }
        let mut txt_hash = self.hash(txt, self.m);
        if self.pat_hash == txt_hash && self.check(txt, 0) {
            return Some(0);
        }
        for i in self.m .. txt.len() {
            txt_hash = (txt_hash + self.q - self.rm * txt.char_at(i-self.m) as usize % self.q) % self.q;
            txt_hash = (txt_hash * self.r + txt.char_at(i) as usize) % self.q;

            let offset = i - self.m + 1;
            if self.pat_hash == txt_hash { // diffs here
                return Some(offset);
            }
        }
        None
    }

    fn long_random_prime() -> usize {
        // TODO: max bits 31, use random prime generator
        5800079
    }
}

#[test]
fn test_rabin_karp() {
    let pat = "abracadabra";
    let text = "abacadabrabracabracadabrabrabracad";
    let bm = RabinKarp::new(pat);
    assert!(bm.search(text).map_or(false, |pos| text[pos..].starts_with(pat)));
    assert!(bm.search_monte_carlo(text).map_or(false, |pos| text[pos..].starts_with(pat)));
    assert_eq!(bm.search("zzzzz"), None);
}
