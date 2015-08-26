use std::ops;
use std::io::{Read, Write, Result};
use byteorder::{ReadBytesExt, WriteBytesExt};
pub use self::Bit::{One, Zero};

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
    bits: u8                    // has n bits in byte
}

impl<W: Write> Drop for BitWriter<W> {
    fn drop(&mut self) {
        if self.bits != 0 {
            println!("warning: bit stream length should be a module of 8");
            println!("warning: zero padding added to {} with bits {}", self.byte, self.bits);
            self.inner.write_u8(self.byte << (8-self.bits)).unwrap();
            self.inner.flush().unwrap();
        }
    }
}

impl<W: Write> BitWriter<W> {
    pub fn new(inner: W) -> BitWriter<W> {
        BitWriter {
            inner: inner,
            byte: 0u8,
            bits: 0u8
        }
    }

    pub fn write_bit(&mut self, bit: Bit) -> Result<()> {
        self.byte = (self.byte << 1) + bit.to_u8();
        self.bits += 1;
        if self.bits == 8 {
            try!(self.inner.write_u8(self.byte));
            self.byte = 0;
            self.bits = 0;
        }
        Ok(())
    }

    pub fn write_u8(&mut self, s: u8) -> Result<()> {
        let ofs = (8 - self.bits) % 8;
        try!(self.inner.write_u8((self.byte << ofs) | (s >> self.bits)));
        self.byte = s & (0xff >> ofs);
        Ok(())
    }

    pub fn write_u16(&mut self, s: u16) -> Result<()> {
        try!(self.write_u8((s >> 8) as u8));
        self.write_u8((s & 0xff) as u8)
    }

    pub fn write_u32(&mut self, s: u32) -> Result<()> {
        try!(self.write_u16((s >> 16) as u16));
        self.write_u16((s & 0xffff) as u16)
    }

    pub fn flush_bits(&mut self) -> Result<()> {
        assert!(self.bits == 0 && self.byte == 0, "must be a modulo of 8");
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


#[allow(unused_must_use)]
#[test]
fn test_bit_writer() {
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
        assert!(w.write_u16(0xffff).is_ok());
        assert!(w.write_bit(Zero).is_ok());
        assert!(w.write_bit(Zero).is_ok());
    }
    // println!("D {:08b}", buf[0]);
    // println!("D {:08b}", buf[1]);
    // println!("D {:08b}", buf[2]);
    // println!("D {:08b}", buf[3]);
    // println!("D {:08b}", buf[4]);
    assert_eq!(buf[0], 0b01111111);
    assert_eq!(buf[1], 0b10101111);
    assert_eq!(buf[2], 0b11111011);
    assert_eq!(buf[3], 0b11111111);
    assert_eq!(buf[4], 0b11111100);
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
