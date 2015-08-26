use std::io::{Read, Write, Result};
use std::ops;
use byteorder::{ReadBytesExt, WriteBytesExt};
use self::Bit::{Zero, One};

pub mod huffman;


const RADIX: usize = 256;
// const LgR: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bit {
    Zero,
    One
}

impl Bit {
    pub fn to_u8(&self) -> u8 {
        match *self  {
            Zero => 0,
            One  => 1
        }
    }

    pub fn from_u8(bit: u8) -> Self {
        match bit & 1u8{
            0 => Zero,
            1 => One,
            _ => unreachable!()
        }
    }
}

impl ops::Not for Bit {
    type Output = Bit;
    fn not(self) -> Self::Output {
        match self {
            Zero => One,
            One  => Zero
        }
    }
}

pub struct BitWriter<W: Write> {
    inner: W,
    byte: u8,
    offset: u8
}

impl<W: Write> Drop for BitWriter<W> {
    fn drop(&mut self) {
        if self.offset != 0 {
            println!("warning: bit stream length should be a module of 8");
            println!("warning: zero padding added to {} with offset {}", self.byte, self.offset);
            self.inner.write_u8(self.byte << (8-self.offset)).unwrap();
            self.inner.flush().unwrap();
        }
    }
}

impl<W: Write> BitWriter<W> {
    pub fn new(inner: W) -> BitWriter<W> {
        BitWriter {
            inner: inner,
            byte: 0u8,
            offset: 0u8
        }
    }

    pub fn write_bit(&mut self, bit: Bit) -> Result<()> {
        self.byte = (self.byte << 1) + bit.to_u8();
        self.offset += 1;
        if self.offset == 8 {
            try!(self.inner.write_u8(self.byte));
            self.byte = 0;
            self.offset = 0;
        }
        Ok(())
    }

    pub fn write_u8(&mut self, s: u8) -> Result<()> {
        let ofs = (8 - self.offset) % 8;
        try!(self.inner.write_u8((self.byte << ofs) + (s >> self.offset)));
        self.byte = s & (0xff >> self.offset);
        Ok(())
    }

    pub fn flush_bits(&mut self) -> Result<()> {
        assert!(self.offset == 0 && self.byte == 0, "must be a modulo of 8");
        try!(self.inner.flush());
        Ok(())
    }
}

pub struct BitReader<R: Read> {
    inner: R,
    byte: u8,
    offset: u8
}

impl<R: Read> Drop for BitReader<R> {
    fn drop(&mut self) {
        if self.offset != 0 {
            println!("warning: bit stream length should be a module of 8");
            println!("warning: bit value remain to be read: {}",
                     self.byte & (0xff >> self.offset));
        }
    }
}

impl<R: Read> BitReader<R> {
    pub fn new(inner: R) -> BitReader<R> {
        BitReader {
            inner: inner,
            byte: 0,
            offset: 0
        }
    }

    pub fn read_bit(&mut self) -> Result<Bit> {
        if self.offset == 0 {
            self.byte = try!(self.inner.read_u8());
            self.offset = 8;    // 8 - 1
        }
        self.offset -= 1;
        let bit = Bit::from_u8(self.byte >> self.offset);
        Ok(bit)
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        let offs = 8 - self.offset;
        if self.offset == 0 {
            Ok(try!(self.inner.read_u8()))
        } else {
            let next = try!(self.inner.read_u8());
            let out = (self.byte << offs) + (next >> self.offset);
            self.byte = next;
            Ok(out)
        }
    }
}

// FIXME: better API
pub trait RunLengthEncoding {
    fn compress(&self, &mut [u8]) -> Result<usize>;
    fn decompress(&self, &mut [u8]) -> Result<usize>;
}

impl RunLengthEncoding for [u8] {
    fn decompress(&self, buf: &mut [u8]) -> Result<usize> {
        let mut bit = Zero;
        let mut out = BitWriter::new(buf);
        let mut nwrite = 0;
        for run in self.iter() {
            for _ in 0 .. *run {
                try!(out.write_bit(bit));
            }
            bit = !bit;
            nwrite += *run as usize;
        }
        try!(out.flush_bits());
        Ok(nwrite/8)
    }

    fn compress(&self, buf: &mut [u8]) -> Result<usize>{
        let mut run = 0u8;
        let mut old = Zero;
        let mut ret = Ok(0);
        let mut inbit = BitReader::new(self);
        // mut :(
        let mut buf = buf;
        loop {
            let b = inbit.read_bit(); // .or_else(|e| e.kind() == io::ErrorKind);
            if b.is_err()  {
                // ret = b.map(|_| 0);
                break;
            }
            let b = b.unwrap();
            if b != old {
                try!(buf.write_u8(run));
                run = 1;
                old = !old;
                let _ = ret.as_mut().map(|n| *n += 1);
            } else {
                if run == (RADIX - 1) as u8 {
                    try!(buf.write_u8(run));
                    run = 0;
                    try!(buf.write_u8(run));
                    let _ = ret.as_mut().map(|n| *n += 2);
                }
                run += 1;
            }
        }
        try!(buf.write_u8(run));
        let _ = ret.as_mut().map(|n| *n += 1);
        ret
    }
}

#[allow(unused_must_use)]
#[test]
fn test_bit_writer() {
    println!("");

    let mut buf: Vec<u8> = Vec::with_capacity(8);
    {
        let buf = &mut buf;
        let mut w = BitWriter::new(buf);
        assert!(w.write_bit(Zero).is_ok());
        assert!(w.write_u8(0xff).is_ok());
        assert!(w.write_bit(Zero).is_ok());
        assert!(w.write_bit(One).is_ok());
        assert!(w.write_bit(Zero).is_ok());
        assert!(w.write_u8(0xff).is_ok());
        assert!(w.write_bit(One).is_ok());
        assert!(w.write_bit(Zero).is_ok());
        assert!(w.write_bit(Zero).is_ok());
        assert!(w.write_bit(Zero).is_ok());
    }
    // println!("D {:08b}", buf[0]);
    // println!("D {:08b}", buf[1]);
    // println!("D {:08b}", buf[2]);
    assert_eq!(buf[0], 0b01111111);
    assert_eq!(buf[1], 0b10101111);
    assert_eq!(buf[2], 0b11111000);
}



#[allow(unused_must_use)]
#[test]
fn test_bit_reader() {
    let a: Vec<u8> = vec![0b01111111, 0b10000001];
    let b: &[u8] = &a;
    let mut r = BitReader::new(b);
    assert_eq!(r.read_bit().unwrap(), Zero);
    assert_eq!(r.read_u8().unwrap(), 0xff);
    for _ in 0 .. 6 {
        assert_eq!(r.read_bit().unwrap(), Zero);
    }
    assert_eq!(r.read_bit().unwrap(), One);
    assert!(r.read_bit().is_err());
}

#[test]
fn test_rle() {
    let a: &'static [u8] = b"welcometochina";

    let mut buf1 = Vec::with_capacity(1024);
    unsafe { buf1.set_len(1024); }
    let mut buf2 = buf1.clone();

    let val1 = a.compress(&mut buf1).unwrap();
    // println!("got => {:?}", &buf1[..val1]);

    let val2 = buf1[..val1].decompress(&mut buf2).unwrap();
    // println!("got => {:?}", &buf2[..val2]);

    assert!(val1 != val2);
    assert_eq!(a, &buf2[..val2]);
}
