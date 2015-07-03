use std::iter;
use super::StackOfStrings;

const INITIAL_STACK_CAPACITY: usize = 1;

pub struct ResizingArrayStackOfStrings {
    s: Vec<Option<String>>,
    n: usize
}

impl ResizingArrayStackOfStrings {
    pub fn with_capacity(capacity: usize) -> ResizingArrayStackOfStrings {
        let storage = iter::repeat(None).take(capacity).collect();

        ResizingArrayStackOfStrings {
            s: storage,
            n: 0
        }
    }

    fn resize(&mut self, capacity: usize) {
        let mut new_storage: Vec<Option<String>> = iter::repeat(None).take(capacity).collect();
        for i in 0 .. self.n {
            new_storage[i] = self.s[i].take();
        }
        self.s = new_storage;
    }
}

impl StackOfStrings for ResizingArrayStackOfStrings {
    fn new() -> ResizingArrayStackOfStrings {
        let storage = iter::repeat(None).take(INITIAL_STACK_CAPACITY).collect();

        ResizingArrayStackOfStrings {
            s: storage,
            n: 0
        }
    }

    fn is_empty(&self) -> bool {
        self.n == 0
    }

    fn push(&mut self, item: String) {
        let len = self.s.len();
        if self.n == len {
            self.resize(2 * len);
        }
        self.s[self.n] = Some(item);
        self.n += 1;
    }

    fn pop(&mut self) -> String {
        self.n -= 1;
        let cell = self.s[self.n].take();
        let len = self.s.len();
        if self.n > 0 && self.n == len / 4 {
            self.resize(len / 2);
        }
        cell.unwrap()
    }
}


#[test]
fn test_resizing_array_stack() {
    let mut stack: ResizingArrayStackOfStrings = StackOfStrings::new();

    let mut result = "to be not that or be".split(' ');

    for s in "to be or not to - be - - that - - - is".split(' ') {
        if s == "-" {
            assert_eq!(&stack.pop(), result.next().unwrap())
        } else {
            stack.push(s.into())
        }
    }
}
