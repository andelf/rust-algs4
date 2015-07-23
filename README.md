rust-algs4
==========

[![Build Status](https://travis-ci.org/andelf/rust-algs4.svg?branch=master)](https://travis-ci.org/andelf/rust-algs4)  [![Crates.io](http://meritbadge.herokuapp.com/algs4)](https://crates.io/crates/algs4)   [![MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://github.com/andelf/rust-algs4/blob/master/LICENSE)

Algorithms, 4ed. MOOC in Coursera.

algs4partI-008 Code & Assignments in Rust.

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

### Programming Assignment 5: Kd-Trees

- ``Point2D``, ``RectHV``, ``PointSet``: ``src/geometric_search/primitive.rs``
- ``KdTree``: ``src/geometric_search/kd_tree.rs``
- ``Program``: ``examples/kdtree.rs``
- ``Benchmark``: ``benches/geometric_search.rs``

```
ᐅ cargo run --example kdtree < priv/input100K.txt
     Running `target/debug/examples/kdtree`
got 100000 points
in rect [0.4, 0.6] x [0.4, 0.6]
PointSet => 3975
KdTree   => 3975
nearest to (0.9, 0.6)
PointSet => Some(Point2D { x: 0.9001859999999999, y: 0.5974529999999999 }) d = 0.002553782488780193
KdTree   => Some(Point2D { x: 0.9001859999999999, y: 0.5974529999999999 }) d = 0.002553782488780193

ᐅ cargo bench
test bench_brute_force_nearest      ... bench:      14,265 ns/iter (+/- 3,196)
test bench_brute_force_range_search ... bench:      14,660 ns/iter (+/- 2,676)
test bench_kd_tree_nearest          ... bench:       4,351 ns/iter (+/- 7,388)
test bench_kd_tree_range_search     ... bench:       1,475 ns/iter (+/- 855)
```



## Description

TODO: add file description
