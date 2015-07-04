use super::{StackOfStrings, Stack};

struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    /// work around for moved value
    fn into_item_and_next(self) -> (T, Option<Box<Node<T>>>) {
        (self.item, self.next)
    }
}

pub struct LinkedStackOfStrings {
    first: Option<Box<Node<String>>>
}

impl StackOfStrings for LinkedStackOfStrings {
    fn new() -> LinkedStackOfStrings {
        LinkedStackOfStrings { first: None }
    }

    fn is_empty(&self) -> bool {
        self.first.is_none()
    }

    fn push(&mut self, item: String) {
        // Takes the value out of the option, leaving a None in its place.
        let old_first = self.first.take();
        let first = Node {
            item: item,
            next: old_first
        };
        self.first = Some(Box::new(first));
    }

    fn pop(&mut self) -> String {
        let old_first = self.first.take();
        let (item, first) = old_first.unwrap().into_item_and_next();
        self.first = first;
        item
    }
}


pub struct LinkedStack<T> {
    first: Option<Box<Node<T>>>
}

impl<T> Stack<T> for LinkedStack<T> {
    fn new() -> LinkedStack<T> {
        LinkedStack { first: None }
    }

    fn is_empty(&self) -> bool {
        self.first.is_none()
    }

    fn push(&mut self, item: T) {
        // Takes the value out of the option, leaving a None in its place.
        let old_first = self.first.take();
        let first = Node {
            item: item,
            next: old_first
        };
        self.first = Some(Box::new(first));
    }

    fn pop(&mut self) -> T {
        let old_first = self.first.take();
        let (item, first) = old_first.unwrap().into_item_and_next();
        self.first = first;
        item
    }
}






#[test]
fn test_linked_stack_of_strings() {
    let mut stack: LinkedStackOfStrings = StackOfStrings::new();

    let mut result = "to be not that or be".split(' ');

    for s in "to be or not to - be - - that - - - is".split(' ') {
        if s == "-" {
            assert_eq!(&stack.pop(), result.next().unwrap())
        } else {
            stack.push(s.into())
        }
    }
}


#[test]
fn test_linked_stack() {
    let mut stack: LinkedStack<i32> = Stack::new();

    let result = [1, 3, 4, 3, 5, 3];
    let mut rit = result.iter();

    for s in vec![1, 3, 5, 4, 1, 0, 3, 0, 0, 3, 0, 0, 0, 4] {
        if s == 0 {
            assert_eq!(&stack.pop(), rit.next().unwrap())
        } else {
            stack.push(s)
        }
    }
}
