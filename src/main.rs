#![feature(core)]
#![feature(io)]
#![feature(env)]
#![feature(path)]

extern crate crc64;
use crc64::crc64;
use std::env;
use std::old_io::{BufferedReader, File};

fn main() {
    let mut args = env::args();
    let (len,_) = args.size_hint();
    let prog = args.next().unwrap();

    if len == 1 {
        println!("Usage: {} [list of files]", prog);
        return
    }

    for f in args {
        let mut crc : u64 = 0;
        let file = File::open(&Path::new(f.to_string()));
        let mut reader = BufferedReader::new(file);

        let mut error = false;
        loop {
            let mut buf = [0; 100];
            match reader.read(buf.as_mut_slice()) {
                Err(e) => {
                    if e.kind != std::old_io::EndOfFile {
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
