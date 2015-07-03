use std::iter;
use super::StackOfStrings;

const STACK_CAPACITY: usize = 1024;

pub struct FixedCapacityStackOfStrings {
    s: Vec<Option<String>>,
    n: usize
}

impl FixedCapacityStackOfStrings {
    pub fn with_capacity(capacity: usize) -> FixedCapacityStackOfStrings {
        let storage = iter::repeat(None).take(capacity).collect();

        FixedCapacityStackOfStrings {
            s: storage,
            n: 0
        }
    }
}

impl StackOfStrings for FixedCapacityStackOfStrings {
    fn new() -> FixedCapacityStackOfStrings {
        let storage = iter::repeat(None).take(STACK_CAPACITY).collect();

        FixedCapacityStackOfStrings {
            s: storage,
            n: 0
        }
    }

    fn is_empty(&self) -> bool {
        self.n == 0
    }

    fn push(&mut self, item: String) {
        self.s[self.n] = Some(item);
        self.n += 1;
    }

    fn pop(&mut self) -> String {
        self.n -= 1;
        let cell = self.s[self.n].take();
        cell.unwrap()
    }
}


#[test]
fn test_fixed_capacity_stack() {
    let mut stack: FixedCapacityStackOfStrings = StackOfStrings::new();

    let mut result = "to be not that or be".split(' ');

    for s in "to be or not to - be - - that - - - is".split(' ') {
        if s == "-" {
            assert_eq!(&stack.pop(), result.next().unwrap())
        } else {
            stack.push(s.into())
        }
    }
}
