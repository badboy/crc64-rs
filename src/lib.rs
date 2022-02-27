//! Calculate the crc64 checksum of the given data, starting with the given crc.
//!
//! Implements the CRC64 used by Redis, which is the variant with "Jones" coefficients and init value of 0.
//!
//! Specification of this CRC64 variant follows:
//!
//! ```text
//! Name: crc-64-jones
//! Width: 64 bites
//! Poly: 0xad93d23594c935a9
//! Reflected In: True
//! Xor_In: 0xffffffffffffffff
//! Reflected_Out: True
//! Xor_Out: 0x0
//! Check("123456789"): 0xe9c6d914c4b8d9ca
//! ```
//!
//! Example:
//!
//! ```rust
//! let cksum = crc64::crc64(0, "123456789".as_bytes());
//! assert_eq!(16845390139448941002, cksum);
//! ```

use std::io::{self, Write};

use crc_table::CRC64_TAB;

mod crc_table;

fn to_u64(data: &[u8]) -> u64 {
    debug_assert!(data.len() == 8);
    let arr: [u8; 8] = data.try_into().expect("incorrect length");
    u64::from_le_bytes(arr)
}

pub fn crc64(crc: u64, data: &[u8]) -> u64 {
    let mut crc = crc;
    let mut len = data.len();
    let mut offset = 0usize;

    while len >= 8 {
        crc ^= to_u64(&data[offset..(offset + 8)]);
        crc = CRC64_TAB[7][(crc & 0xff) as usize]
            ^ CRC64_TAB[6][((crc >> 8) & 0xff) as usize]
            ^ CRC64_TAB[5][((crc >> 16) & 0xff) as usize]
            ^ CRC64_TAB[4][((crc >> 24) & 0xff) as usize]
            ^ CRC64_TAB[3][((crc >> 32) & 0xff) as usize]
            ^ CRC64_TAB[2][((crc >> 40) & 0xff) as usize]
            ^ CRC64_TAB[1][((crc >> 48) & 0xff) as usize]
            ^ CRC64_TAB[0][(crc >> 56) as usize];

        offset += 8;
        len -= 8;
    }

    while len > 0 {
        crc = CRC64_TAB[0][((crc ^ data[offset] as u64) & 0xff) as usize] ^ (crc >> 8);
        offset += 1;
        len -= 1;
    }

    crc
}

pub struct Crc64 {
    crc64: u64,
}

impl Crc64 {
    pub fn new() -> Crc64 {
        Crc64 { crc64: 0 }
    }
    pub fn get(&self) -> u64 {
        self.crc64
    }
}

impl Default for Crc64 {
    fn default() -> Self {
        Self::new()
    }
}

impl Write for Crc64 {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.crc64 = crc64(self.crc64, buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn test_crc64_works() {
    assert_eq!(0xe9c6d914c4b8d9ca, crc64(0, "123456789".as_bytes()))
}

#[test]
fn test_crc64_write() {
    let step1 = "12345".as_bytes();
    let step2 = "6789".as_bytes();
    let value1 = 17326901458626182669;
    let value2 = 16845390139448941002;
    assert_eq!(value1, crc64(0, step1));
    assert_eq!(value2, crc64(value1, step2));

    let mut crc = Crc64::new();
    assert_eq!(crc.write(step1).unwrap(), step1.len());
    assert_eq!(value1, crc.get());
    assert_eq!(crc.write(step2).unwrap(), step2.len());
    assert_eq!(value2, crc.get());
}
