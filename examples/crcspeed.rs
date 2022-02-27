use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() == 1 {
        println!("Usage: {} [list of files]", args[0]);
        panic!();
    }

    let f = &args[1];
    let mut crc : u64 = 0;
    let content = fs::read(f).expect("can't read file");

    let sz = content.len();
    let size_mb = sz as f64 / 1024.0 / 1024.0;

    println!("CRC for {:.2} MB file", size_mb);

    let start = Instant::now();
    crc = crc64::crc64(crc, content.as_slice());
    let elapsed = start.elapsed();

    let total_time_seconds = elapsed.as_secs();
    let speed = size_mb / total_time_seconds as f64;

    println!("{:x}  {}", crc, f);
    println!("{} seconds at {:.2} MB/s", total_time_seconds, speed);
}
