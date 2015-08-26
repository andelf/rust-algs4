use std::cmp;
use std::iter;
use self::Node::*;
use super::{BitWriter, BitReader, Bit};
use super::Bit::{Zero, One};
use std::io::{Result, Write, Read};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};

use adivon::MinPQ;

const RADIX: usize = 256;

#[derive(PartialEq, Debug)]
enum Node {
    Leaf { freq: usize, ch: u8 },
    Branch {
        freq: usize,
        left: Option<Box<Node>>,
        right: Option<Box<Node>>
    }
}

impl Node {
    fn new_leaf(freq: usize, ch: u8) -> Node {
        Node::Leaf { ch: ch, freq: freq }
    }

    fn new_branch(freq: usize, left: Option<Node>, right: Option<Node>) -> Node {
        Node::Branch {
            freq: freq,
            left: left.map(Box::new),
            right: right.map(Box::new)
        }
    }

    fn freq(&self) -> usize {
        match *self {
            Node::Leaf { freq, ..}    => freq,
            Node::Branch { freq, .. } => freq

        }
    }

    fn write<W: Write>(x: Option<&Box<Node>>, w: &mut BitWriter<W>) -> Result<()> {
        if x.is_some() {
            match **x.unwrap() {
                Node::Leaf { ch, .. } => {
                    try!(w.write_bit(One));
                    try!(w.write_u8(ch));
                },
                Node::Branch { ref left, ref right, .. } => {
                    try!(w.write_bit(Zero));
                    try!(Node::write(left.as_ref(), w));
                    try!(Node::write(right.as_ref(), w));
                }
            }
        }
        Ok(())
    }

    fn read<R: Read>(r: &mut BitReader<R>) -> Result<Option<Box<Node>>> {
        let bit = try!(r.read_bit());
        match bit {
            One => {
                Ok(Some(Box::new(Node::new_leaf(0, try!(r.read_u8())))))
            },
            Zero => {
                let left = try!(Node::read(r)).map(|n| *n);
                let right = try!(Node::read(r)).map(|n| *n);
                Ok(Some(Box::new(Node::new_branch(0, left, right))))
            }
        }
    }

}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<cmp::Ordering> {
        self.freq().partial_cmp(&other.freq())
    }
}

#[derive(Debug)]
pub struct Huffman {
    root: Option<Box<Node>>,
    code_table: Vec<Vec<Bit>>
}

impl Huffman {
    fn from_freq(freq: &[usize]) -> Huffman {
        let root = Some(Box::new(Huffman::build_trie(freq)));
        let mut code_table = iter::repeat(Vec::new())
            .take(RADIX).collect::<Vec<Vec<Bit>>>();
        Huffman::build_code(&mut code_table, root.as_ref(), Vec::new());

        Huffman {
            root: root,
            code_table: code_table
        }
    }

    fn read<R: Read>(r: &mut BitReader<R>) -> Huffman {
        let root = Node::read(r).unwrap();
        let mut code_table = iter::repeat(Vec::new())
            .take(RADIX).collect::<Vec<Vec<Bit>>>();
        Huffman::build_code(&mut code_table, root.as_ref(), Vec::new());
        Huffman {
            root: root,
            code_table: code_table
        }
    }

    fn next_u8<R: Read>(&self, r: &mut BitReader<R>) -> Result<u8> {
        let root = self.root.as_ref();
        let mut x = root;
        while let &Node::Branch { ref left, ref right, .. } = x.map(|n| &**n).unwrap() {
            let bit = r.read_bit().unwrap();
            if bit == Zero {
                x = left.as_ref();
            } else {
                x = right.as_ref();
            }
        }

        if let &Node::Leaf { ch, .. } = x.map(|n| &**n).unwrap() {
            Ok(ch)
        } else {
            println!("fuck");
            Ok(0)
        }
    }

    fn code_for(&self, ch: u8) -> &[Bit] {
        &self.code_table[ch as usize]
    }

    fn write_trie<W: Write>(&self, w: &mut BitWriter<W>) {
        Node::write(self.root.as_ref(), w).ok().expect("write tire");
    }

    fn build_trie(freq: &[usize]) -> Node {
        let mut pq = MinPQ::new();

        for i in 0 .. RADIX {
            if freq[i] > 0 {
                pq.insert(Node::new_leaf(freq[i], i as u8));
            }
        }

        if pq.size() == 1 {
            if freq[0] == 0 {
                pq.insert(Node::new_leaf(0, 0 as u8));
            } else {
                pq.insert(Node::new_leaf(0, 1 as u8));
            }
        }

        // merge into smallest trees
        while pq.size() > 1 {
            let left = pq.del_min();
            let right = pq.del_min();
            let freq = left.as_ref().map_or(0, Node::freq) +
                right.as_ref().map_or(0, Node::freq);
            let parent = Node::new_branch(freq, left, right);
            pq.insert(parent);
        }

        pq.del_min().unwrap()
    }

    fn build_code(st: &mut Vec<Vec<Bit>>, x: Option<&Box<Node>>, mut s: Vec<Bit>) {
        if x.is_some() {
            match **x.unwrap() {
                Node::Branch { ref left, ref right, .. } => {
                    {
                        let mut s = s.clone();
                        s.push(Bit::Zero);
                        Huffman::build_code(st, left.as_ref(), s);
                    }
                    s.push(Bit::One);
                    Huffman::build_code(st, right.as_ref(), s);
                }
                Node::Leaf { ch, .. } => {
                    st[ch as usize] = s
                }
            }
        }
    }
}

pub trait HuffmanEncoding {
    fn compress(&self, &mut [u8]) -> Result<usize>;
    fn decompress(&self, &mut [u8]) -> Result<usize>;
}

impl HuffmanEncoding for [u8] {
    fn compress(&self, buf: &mut [u8]) -> Result<usize> {
        // workaround
        let inlen = buf.len();
        let mut buf = buf;
        let mut freq = iter::repeat(0).take(RADIX).collect::<Vec<usize>>();
        self.iter().map(|&c| freq[c as usize] += 1).count();

        // build Huffman trie
        let huffman = Huffman::from_freq(&freq);
        {
            let mut out = BitWriter::new(&mut buf);
            // decoding trie
            huffman.write_trie(&mut out);
        }
        // for (i, v) in huffman.code_table.iter().enumerate() {
        //     if !v.is_empty() {
        //         println!("{} => {:?}", i as u8 as char, v);
        //     }
        // }

        // length of original message
        try!(buf.write_u32::<LittleEndian>(self.len() as u32));

        {
            let mut out = BitWriter::new(&mut buf);
            for &i in self.iter() {
                for &b in huffman.code_for(i) {
                    try!(out.write_bit(b));
                }
            }
        }
        let outlen = buf.len();
        Ok(inlen - outlen)
    }

    fn decompress(&self, buf: &mut [u8]) -> Result<usize> {
        let mut inbuf = self;

        let huffman = {
            let mut inbits = BitReader::new(&mut inbuf);
            Huffman::read(&mut inbits)
        };

        // for (i, v) in huffman.code_table.iter().enumerate() {
        //     if !v.is_empty() {
        //         println!("{} => {:?}", i as u8 as char, v);
        //     }
        // }
        // length of original message
        let len = try!(inbuf.read_u32::<LittleEndian>()) as usize;

        let mut inbits = BitReader::new(&mut inbuf);
        for i in 0 .. len {
            let r = huffman.next_u8(&mut inbits);
            buf[i] = r.unwrap()
        }

        Ok(len)
    }
}


#[test]
fn test_huffman() {
    let a: &'static [u8] = b"she sells seashells on the seashore the shells she sells are surely seashell";

    // println!("orig len = {:?}", a.len());
    // println!("orig => {:?}", a);
    let orig_len = a.len();
    let mut buf1 = Vec::with_capacity(1024);
    unsafe { buf1.set_len(1024); }
    let mut buf2 = buf1.clone();

    let len = a.compress(&mut buf1).unwrap();
    // println!("compressed len => {}", len);
    // println!("compressed => {:?}", &buf1[..len]);

    // compress ratio
    assert!(len as f64 / orig_len as f64 <= 0.99);

    // for c in buf1[..len].iter() {
    //     println!("{:08b}", c);
    // }

    let len2 = buf1[..len].decompress(&mut buf2).unwrap();

    assert_eq!(len2, orig_len);
    assert_eq!(&String::from_utf8_lossy(&buf2[..len2]),
               "she sells seashells on the seashore the shells she sells are surely seashell");

}
