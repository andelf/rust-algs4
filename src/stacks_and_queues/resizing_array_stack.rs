use std::iter;
use std::iter::Iterator;
use std::marker::PhantomData;
use super::{StackOfStrings, Stack};

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
        ResizingArrayStackOfStrings::with_capacity(INITIAL_STACK_CAPACITY)
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


pub struct ResizingArrayStack<T> {
    s: Vec<Option<T>>,
    n: usize
}

impl<T> ResizingArrayStack<T> {
    pub fn with_capacity(capacity: usize) -> ResizingArrayStack<T> {
        let mut storage = Vec::with_capacity(capacity);
        for _ in 0 .. capacity {
            storage.push(None);
        }

        ResizingArrayStack {
            s: storage,
            n: 0
        }
    }

    fn resize(&mut self, capacity: usize) {
        let mut new_storage = Vec::with_capacity(capacity);
        for i in 0 .. capacity {
            if i < self.n {
                new_storage.push(self.s[i].take())
            } else {
                new_storage.push(None);
            }
        }
        self.s = new_storage;
    }
}

impl<T> Stack<T> for ResizingArrayStack<T> {
    fn new() -> ResizingArrayStack<T> {
        ResizingArrayStack::with_capacity(INITIAL_STACK_CAPACITY)
    }

    fn is_empty(&self) -> bool {
        self.n == 0
    }

    fn push(&mut self, item: T) {
        let len = self.s.len();
        if self.n == len {
            self.resize(2 * len);
        }
        self.s[self.n] = Some(item);
        self.n += 1;
    }

    fn pop(&mut self) -> T {
        self.n -= 1;
        let cell = self.s[self.n].take();
        let len = self.s.len();
        if self.n > 0 && self.n == len / 4 {
            self.resize(len / 2);
        }
        cell.unwrap()
    }
}

pub struct Iter<'a, T: 'a> {
    ptr: *const Option<T>,
    end: *const Option<T>,
    _marker: PhantomData<&'a T>
}

unsafe impl<'a, T: Sync> Sync for Iter<'a, T> {}
unsafe impl<'a, T: Sync> Send for Iter<'a, T> {}

// FIXME: maybe error with zero-sized type
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        unsafe {
            if self.ptr < self.end {
                None
            } else {
                self.ptr = self.ptr.offset(-1);
                (*self.ptr).as_ref()
            }
        }
    }
}

impl<'a, T> ResizingArrayStack<T> {
    pub fn iter(&'a self) -> Iter<'a, T> {
        Iter {
            // first empty
            ptr: &self.s[self.n],
            end: &self.s[0],
            _marker: PhantomData
        }
    }
}


#[test]
fn test_resizing_array_stack_of_strings() {
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

#[test]
fn test_resizing_array_stack() {
    let mut stack: ResizingArrayStack<i32> = Stack::new();

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

#[test]
fn test_resizing_array_stack_iter() {
    let mut stack: ResizingArrayStack<i32> = Stack::new();

    let result = [3, 3, 1];
    let mut rit = result.iter();

    for s in vec![1, 3, 5, 0, 1, 0, 3] {
        if s == 0 {
            stack.pop();
        } else {
            stack.push(s);
        }
    }

    for v in stack.iter() {
        println!("v => {:?}", v);
        assert_eq!(v, rit.next().unwrap())
    }
}


#[test]
fn test_resizing_array_stack_iter_string() {
    let mut stack: ResizingArrayStack<String> = Stack::new();

    let mut result = "is that or be to".split(' ');

    for s in "to be or not to - be - - that is".split(' ') {
        if s == "-" {
            stack.pop();
        } else {
            stack.push(s.into());
        }
    }

    for v in stack.iter() {
        assert_eq!(v, result.next().unwrap())
    }
}
