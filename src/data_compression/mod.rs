use std::io::Result;
use byteorder::WriteBytesExt;
pub use self::bits::{Bit, Zero, One};
pub use self::bits::{BitReader, BitWriter};

pub mod bits;
pub mod huffman;
pub mod lzw;


const RADIX: usize = 256;
// const LgR: usize = 8;

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
