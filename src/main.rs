#![feature(slicing_syntax)]

extern crate crc64;
use crc64::crc64;
use std::os;
use std::io::{BufferedReader, File};

fn main() {
    let args = os::args();
    if args.len() == 1 {
        println!("Usage: {} [list of files]", args[0]);
        return
    }

    for f in args[1..].iter() {
        let mut crc : u64 = 0;
        let file = File::open(&Path::new(f.to_string()));
        let mut reader = BufferedReader::new(file);

        let mut error = false;
        loop {
            let mut buf = [0; 100];
            match reader.read(buf.as_mut_slice()) {
                Err(e) => {
                    if e.kind != std::io::EndOfFile {
                        error = true;
                        print!("error reading '{}': {}", f, e);
                    }
                    break;
                },
                Ok(nread) => crc = crc64::crc64(crc, buf[..nread].as_slice())
            }
        }

        if error == false {
            println!("{:x}  {}", crc, f);
        }
    }
}
