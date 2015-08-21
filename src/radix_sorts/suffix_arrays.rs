use std::iter;
use std::iter::FromIterator;

const CUTOFF: usize = 5;


pub struct SuffixArray {
    text: Vec<char>,
    index: Vec<usize>,
    n: usize
}

impl SuffixArray {
    pub fn new(s: &str) -> SuffixArray {
        let n = s.len();
        let text = s.chars().chain(iter::once(0 as char)).collect::<Vec<char>>();
        let index = (0..n).collect();

        let mut sa = SuffixArray {
            text: text,
            index: index,
            n: n
        };
        sa.sort(0, n-1, 0);
        sa
    }

    // 3-way string quicksort
    fn sort(&mut self, lo: usize, hi: usize, d: usize) {
        // cutoff
        if hi <= lo + CUTOFF {
            self.insertion(lo, hi, d);
            return ;
        }

        let mut lt = lo;
        let mut gt = hi;
        let v = self.text[self.index[lo] + d];
        let mut i = lo + 1;
        while i <= gt {
            let t = self.text[self.index[i] + d];
            if t < v {
                self.index.swap(lt, i);
                lt += 1;
                i += 1;
            } else if t > v {
                self.index.swap(i, gt);
                gt -= 1;
            } else {
                i += 1;
            }
        }

        self.sort(lo, lt-1, d);
        if v > 0 as char {
            self.sort(lt, gt, d+1);
        }
        self.sort(gt+1, hi, d);
    }

    fn insertion(&mut self, lo: usize, hi: usize, d: usize) {
        for i in lo .. hi+1 {
            for j in (0 .. i+1).rev() {
                if !(j > lo && self.less(self.index[j], self.index[j-1], d)) {
                    break;
                }
                self.index.swap(j, j-1);
            }
        }
    }

    fn less(&self, i: usize, j: usize, d: usize) -> bool {
        if i == j {
            return false;
        }
        let mut i = i + d;
        let mut j = j + d;
        while i < self.n && j < self.n {
            if self.text[i] < self.text[j] {
                return true;
            }
            if self.text[i] > self.text[j] {
                return false;
            }
            i += 1;
            j += 1;
        }
        return i > j;
    }

    pub fn length(&self) -> usize {
        self.n
    }

    pub fn index(&self, i: usize) -> usize {
        self.index[i]
    }

    // length of the longest common prefix
    pub fn lcp(&self, i: usize) -> usize {
        if i < 1 || i >= self.n {
            panic!("index out of bounds");
        }
        self.lcp_helper(self.index[i], self.index[i-1])
    }

    fn lcp_helper(&self, mut i: usize, mut j: usize) -> usize {
        let mut length = 0;
        while i < self.n && j < self.n {
            if self.text[i] != self.text[j] {
                return length;
            }
            i += 1;
            j += 1;
            length += 1;
        }
        length
    }

    pub fn select(&self, i: usize) -> String {
        if i >= self.n {
            panic!("index out of bounds");
        }
        FromIterator::from_iter(self.text[self.index[i] .. self.n].iter().map(|c| *c))
    }

    pub fn rank(&self, query: &str) -> usize {
        let mut lo = 0;
        let mut hi = self.n - 1;
        while lo <= hi {
            let mid = lo + (hi-lo)/2;
            let cmp = self.compare(query, self.index[mid]);
            if cmp <0 {
                hi = mid - 1;
            } else if cmp > 0 {
                lo = mid + 1;
            } else {
                return mid;
            }
        }
        return lo;
    }

    fn compare(&self, query: &str, mut i: usize) -> i32 {
        let qlen = query.len();
        let mut j = 0;
        while i < self.n && j <= qlen {
            if query.char_at(j) != self.text[i] {
                return query.char_at(j) as i32 - self.text[i] as i32;
            }
            i += 1;
            j += 1;
        }
        if i < self.n { return -1; }
        if j < qlen { return 1; }
        return 0;
    }
}


#[test]
fn test_suffix_array() {
    let s = "banana";
    // suffix 0 => 5 a
    // suffix 1 => 3 ana
    // suffix 2 => 1 anana
    // suffix 3 => 0 banana
    // suffix 4 => 4 na
    // suffix 5 => 2 nana
    let suffix = SuffixArray::new(s);

    assert_eq!(suffix.select(0), "a");
    assert_eq!(suffix.index(0), 5);

    assert_eq!(suffix.select(4), "na");
    // "anz" should be in 3rd place
    assert_eq!(suffix.rank("anz"), 3);
}
