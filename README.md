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

### Programming Assignment 3: Pattern Recognition

- ``Program``: ``examples/collinear.rs``

```
ᐅ cat /path/to/collinear/input6.txt | cargo run --example collinear
     Running `target/debug/examples/collinear`
# Brute force
(21000, 10000) -> (32000, 10000) -> (18000, 10000) -> (19000, 10000)
(14000, 10000) -> (32000, 10000) -> (18000, 10000) -> (19000, 10000)
(14000, 10000) -> (21000, 10000) -> (18000, 10000) -> (19000, 10000)
(14000, 10000) -> (21000, 10000) -> (32000, 10000) -> (19000, 10000)
(14000, 10000) -> (21000, 10000) -> (32000, 10000) -> (18000, 10000)
# A faster, sorting-based solution
(14000, 10000) -> (18000, 10000) -> (19000, 10000) -> (21000, 10000) -> (32000, 10000)

ᐅ cat /path/to/collinear/input8.txt | cargo run --example collinear
     Running `target/debug/examples/collinear`
# Brute force
(7000, 3000) -> (3000, 7000) -> (0, 10000) -> (10000, 0)
(6000, 7000) -> (14000, 15000) -> (3000, 4000) -> (20000, 21000)
# A faster, sorting-based solution
(10000, 0) -> (7000, 3000) -> (3000, 7000) -> (0, 10000)
(3000, 4000) -> (6000, 7000) -> (14000, 15000) -> (20000, 21000)
```

### Programming Assignment 4: 8 Puzzle

- ``Program``: ``examples/8puzzle.rs``

```
ᐅ time cat /path/to/8puzzle/puzzle01.txt | cargo run --example 8puzzle
     Running `target/debug/examples/8puzzle`
Minimum number of moves = 1
2
 1  0
 3  2

2
 1  2
 3  0

cargo run --example 8puzzle  0.03s user 0.03s system 93% cpu 0.058 total
```

## Description

TODO: add file description
