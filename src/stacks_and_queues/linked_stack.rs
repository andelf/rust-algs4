use super::StackOfStrings;

struct Node {
    item: String,
    next: Option<Box<Node>>
}

impl Node {
    /// work around for moved value
    fn into_item_and_next(self) -> (String, Option<Box<Node>>) {
        (self.item, self.next)
    }
}

pub struct LinkedStackOfStrings {
    first: Option<Box<Node>>
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
        let mut first = Node {
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


#[test]
fn test_linked_stack() {
    let mut stack: LinkedStackOfStrings = StackOfStrings::new();
    for s in "to be or not to - be - - that - - - is".split(' ') {
        if s == "-" {
            println!("{}", stack.pop());
        } else {
            stack.push(s.into())
        }
    }
}
