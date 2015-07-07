extern crate algs4;

use std::env;
use std::io::prelude::*;
use std::io;

use algs4::stacks_and_queues::RandomizedQueue;
use algs4::stacks_and_queues::resizing_array_randomized_queue::ResizingArrayRandomizedQueue;


fn main() {
    let n = env::args().nth(1).unwrap().parse().unwrap();

    let mut input = String::with_capacity(1024);
    io::stdin().read_to_string(&mut input).unwrap();

    let mut queue: ResizingArrayRandomizedQueue<&str> = RandomizedQueue::new();

    for tok in input.split(|c: char| c.is_whitespace() ) {
        if !tok.is_empty() {
            queue.enqueue(tok);
        }
    }

    for _ in 0 .. n {
        println!("{}", queue.dequeue().unwrap());
    }
}
