//! Compress or expand binary input  using LZW
use std::io::{Write, Result};
use std::iter;

use adivon::TernarySearchTrie;

use super::{BitWriter, BitReader};


const R: usize = 256;
const L: usize = 4096;  // 2^12
const W: usize = 12;    // 12-bits

pub trait LZWEncoding {
    fn compress(&self, &mut [u8]) -> Result<usize>;
    fn decompress(&self, &mut [u8]) -> Result<usize>;
}

impl LZWEncoding for [u8] {
    fn compress(&self, buf: &mut [u8]) -> Result<usize> {
        // store as <usize>
        let mut st = TernarySearchTrie::new();
        for i in 0 .. R {
            st.put(&[i as u8], i);
        }
        // R is codeword for EOF
        let mut code = R + 1;

        let mut buf = buf;
        let mut input = self;
        let buflen = buf.len();

        {
            let mut out = BitWriter::new(&mut buf);
            while input.len() > 0 {
                let s = st.longest_prefix_of(input).unwrap();
                let t = s.len();
                try!(out.write_usize(*st.get(s).unwrap() as usize, W));
                if t < input.len() && code < L {
                    st.put(&input[0 .. t+1], code);
                    code += 1;
                }
                input = &input[t..];
            }
            try!(out.write_usize(R, W));
        }
        Ok(buflen - buf.len())
    }
    fn decompress(&self, buf: &mut [u8]) -> Result<usize> {
        let mut inbuf = self;
        let mut st: Vec<Vec<u8>> = Vec::with_capacity(L);

        for i in 0 .. R {
            st.push(vec![i as u8]);
        }

        // R , EOF
        st.push(vec![]);

        let mut buf: &mut [u8] = buf;
        let mut inbits = BitReader::new(&mut inbuf);
        let mut codeword = try!(inbits.read_usize(W));
        if codeword == R {
            return Ok(0);
        }
        let mut val = st[codeword].clone();
        let mut outlen = 0;

        loop {
            outlen += val.len();
            try!(buf.write_all(&val));
            codeword = try!(inbits.read_usize(W));
            if codeword == R {
                break;
            }
            let mut s: Vec<u8> = st[codeword].clone();
            if codeword == st.len() {
                s = val.iter().chain(iter::once(&val[0])).map(Clone::clone).collect();
            }
            if st.len() < L {
                val.push(s[0]);
                st.push(val);
            }
            val = s;
        }
        Ok(outlen)
    }
}


#[test]
fn test_lzw() {
    let a: &'static [u8] = b"she sells seashells on the seashore the shells she sells are surely seashell";

    let a: Vec<u8> = a.iter().cycle().take(2048 * 8).map(|&c| c).collect();

    // println!("orig => {:?}", a);
    let orig_len = a.len();
    let mut buf1 = Vec::with_capacity(2048 * 8);
    unsafe { buf1.set_len(2048 * 8); }
    let mut buf2 = buf1.clone();

    let len = a.compress(&mut buf1).unwrap();

    // compress ratio
    // println!("compress ratio: {:?}", len as f64 / orig_len as f64);
    assert!(len as f64 / orig_len as f64 <= 0.50);

    // for c in buf1[..len].iter() {
    //     println!("{:08b}", c);
    // }
    let len2 = buf1[..len].decompress(&mut buf2).unwrap();

    assert_eq!(len2, orig_len);
    assert_eq!(&buf2[..len2], &a[..]);
}
