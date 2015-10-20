// http://coursera.cs.princeton.edu/algs4/assignments/8puzzle.html
extern crate algs4;

use std::io::prelude::*;
use std::io;
use std::fmt;
use std::cmp::Ordering;

use algs4::sorting::priority_queues::MinPQ;
use algs4::sorting::priority_queues::binary_heaps::BinaryHeapMinPQ;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    blocks: Vec<Vec<usize>>,
    n: usize
}

impl Board {
    pub fn new(blocks: Vec<Vec<usize>>) -> Board {
        assert!(blocks.len() == blocks[0].len());
        let len = blocks.len();
        Board { blocks: blocks, n: len }
    }

    pub fn dimension(&self) -> usize {
        return self.n
    }
    pub fn hamming(&self) -> usize {
        let mut num = 0;
        for i in 0 .. self.n {
            for j in 0 .. self.n {
                if self.blocks[i][j] != i * self.n + j + 1 && self.blocks[i][j] != 0 {
                    num += 1;
                }
            }
        }
        num
    }

    pub fn manhattan(&self) -> usize {
        let mut distance = 0;
        for i in 0 .. self.n {
            for j in 0 .. self.n {
                //let val = i * self.n + j + 1;
                // if val == 0 {
                //     continue;
                // }
                let val = self.blocks[i][j];
                if val != i * self.n + j + 1 && val != 0 {
                    // current block's val is not in position
                    let actual_row = (val - 1) / self.n;
                    let actual_col = (val - 1) % self.n;

                    let dist = (actual_row as isize - i as isize).abs() + (actual_col as isize - j as isize).abs();
                    distance += dist as usize;
                }
            }
        }
        distance
    }

    pub fn is_goal(&self) -> bool {
        self.hamming() == 0
    }

    fn position_of(&self, val: usize) -> (usize, usize) {
        for i in 0 .. self.n {
            for j in 0 .. self.n {
                if self.blocks[i][j] == val {
                    return (i, j)
                }
            }
        }
        (9999, 9999)
    }

    pub fn neighbors(&self) -> Vec<Board> {
        let (row, col) = self.position_of(0);
        let mut positions = Vec::new();
        if row >= 1 {
            positions.push((row-1, col));
        }
        if row < self.n - 1{
            positions.push((row+1, col));
        }
        if col >= 1 {
            positions.push((row, col-1));
        }
        if col < self.n - 1{
            positions.push((row, col+1));
        }

        let mut ret = Vec::new();
        for (r, c) in positions {
            let mut b = self.clone();
            b.blocks[row][col] = b.blocks[r][c];
            b.blocks[r][c] = 0;
            ret.push(b);
        }
        ret
    }

    pub fn twin(&self) -> Board {
        let mut twin = self.clone();
        for i in 0 .. 2{
            if twin.blocks[i][0] != 0 && twin.blocks[i][1] != 0 {
                twin.blocks[i].swap(0, 1);
                break
            }
        }
        twin
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "{}", self.dimension()));
        for row in self.blocks.iter() {
            try!(write!(f, " "));
            for val in row.iter() {
                try!(write!(f, "{:<3}", val));
            }
            try!(write!(f, "\n"));
        }
        Ok(())
    }
}


// impl PartialOrd for Board {
//     fn partial_cmp(&self, other: &Board) -> Option<Ordering> {
//         self.manhattan().partial_cmp(&other.manhattan())
//     }
// }

// a search Node
#[derive(PartialEq)]
pub struct Node {
    board: Board,
    moves: usize,
    operations: Vec<Board>
}

impl Node {
    pub fn new(board: Board, moves: usize, operations: Vec<Board>) -> Node {
        Node {
            board: board,
            moves: moves,
            operations: operations
        }
    }

    #[inline]
    fn priority(&self) -> usize {
        self.board.manhattan() + self.moves
    }

    fn take(self) -> (Board, usize, Vec<Board>) {
        (self.board, self.moves, self.operations)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        self.priority().partial_cmp(&other.priority())
    }
}

pub struct Solver {
    solvable: bool,
    solution: Vec<Board>,
    moves: usize,
}

impl Solver {
    pub fn new(initial: Board) -> Solver {
        let mut pq: BinaryHeapMinPQ<Node> = MinPQ::new();

        pq.insert(Node::new(initial.clone(), 0, Vec::new()));

        let max_iteration = initial.dimension().pow(3);
        let mut visited = Vec::new();

        visited.push(initial.clone());
        while !pq.is_empty() {
            let (b, mut moves, mut operations) = pq.del_min().unwrap().take();
            if operations.contains(&b) {
                continue;
            }

            operations.push(b.clone());

            if b.is_goal() {
                return Solver {
                    solvable: true,
                    solution: operations,
                    moves: moves
                }
            }

            moves += 1;
            //println!("moves => {:2} size={}", moves, visited.len());
            for neighbor in b.neighbors() {
                if !visited.contains(&neighbor) {
                    visited.push(neighbor.clone());
                    pq.insert(Node::new(neighbor, moves, operations.clone()));
                }
            }

            if moves > max_iteration {
                break;
            }

        }

        Solver {
            solvable: false,
            solution: Vec::new(),
            moves: 0
        }
    }
}


impl fmt::Display for Solver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.solvable {
            try!(writeln!(f, "Minimum number of moves = {}", self.moves));
            for m in self.solution.iter() {
                try!(writeln!(f, "{}", m));
            }
        } else {
            try!(writeln!(f, "No solution possible"));
        }
        Ok(())
    }
}


fn main() {
    let mut lines = io::BufReader::new(io::stdin()).lines();
    let n = lines.next().unwrap().unwrap().parse().unwrap();

    let mut blks: Vec<Vec<usize>> = Vec::new();
    for _ in 0 .. n {
        let segs: Vec<usize> = lines.next().unwrap().unwrap().split(' ').
            filter(|s| !s.is_empty()).map(|n| n.parse().unwrap()).collect();
        blks.push(segs);
    }

    let b = Board::new(blks);

    let solver = Solver::new(b);
    println!("{}", solver);

}


#[test]
fn test_solver() {
    // let blks = vec![
    //     vec![8, 1, 3],
    //     vec![4, 0, 2],
    //     vec![7, 6, 5],
    //     ];
    let blks = vec![
        vec![0, 1, 3],
        vec![4, 2, 5],
        vec![7, 8, 6],
        ];
    // unsolveable
    // let blks = vec![
    //     vec![1, 2, 3],
    //     vec![4, 5, 6],
    //     vec![8, 7, 0],
    //     ];

    let b = Board::new(blks);
    println!("block:\n{}", b);
    println!("hamming => {}", b.hamming());
    println!("manhattan => {}", b.manhattan());
    println!("is goal => {}", b.is_goal());

    for i in b.neighbors() {
        println!("neighbor:\n{}", i);
    }

    let solver = Solver::new(b);
    println!("{}", solver);
}
