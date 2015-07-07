rust-algs4
==========

[![Build Status](https://travis-ci.org/andelf/rust-algs4.svg?branch=master)](https://travis-ci.org/andelf/rust-algs4)

Algorithms, 4ed.

algs4partI-008 in Rust.

[Algorithms, Part I](https://class.coursera.org/algs4partI-008)

## How to

```
cargo build
cargo test
cargo bench
```

## Assignments:

### Programming Assignment 1: Percolation

```
ᐅ cargo run --example percolation 2 1000
     Running `target/debug/examples/percolation 2 1000`
mean                    = 0.67175
stddev                  = 0.1159865957358461
95% confidence interval = 0.6645610763167408, 0.6789389236832591
```

### Programming Assignment 2: Randomized Queues and Deques

- ``Dequeue``: ``src/stacks_and_queues/linked_deque.rs``
- ``Randomized queue``: ``src/stacks_and_queues/resizing_array_randomized_queue.rs``
- ``Subset client``: ``examples/subset.rs``

```
ᐅ echo A B C D E F G H I | cargo run --example subset 3
     Running `target/debug/examples/subset 3`
D
G
C
ᐅ echo A B C D E F G H I | cargo run --example subset 3
     Running `target/debug/examples/subset 3`
D
E
H
ᐅ echo AA BB BB BB BB BB CC CC | cargo run --example subset 8
     Running `target/debug/examples/subset 8`
BB
CC
BB
BB
CC
AA
BB
BB
```

## Description

TODO: add file description
