use std::ptr;
use std::mem;

extern crate time;
use time::{Duration, PreciseTime};

const BUF_SIZE : usize = 512 * 1024;
const SAMPLES : usize = 64;

fn main() {
  println!("run,write,read,bytes");

  let mut read_avg = 0.0;
  let mut write_avg = 0.0;

  for i in 0..SAMPLES {
    let (duration_write, duration_read, bytes) =  bench_sequential();
    let us_write = duration_write.num_microseconds().unwrap() as f64;
    let us_read = duration_read.num_microseconds().unwrap() as f64;
    
    let mbps_write = bytes as f64 / (1024. * 1024.) / (us_write / 1000000.0);
    let mbps_read = bytes as f64 / (1024. * 1024.) / (us_read / 1000000.0);

    read_avg = read_avg + mbps_read * (1. / SAMPLES as f64);
    write_avg = write_avg + mbps_write * (1. / SAMPLES as f64);
          
    println!("{},{},{},{}", i,  mbps_write.round(), mbps_read.round(), bytes)
  }

  println!("avg,{},{},", write_avg as usize, read_avg as usize);
}

fn bench_sequential() -> (Duration, Duration, usize) {
  let chunk: [usize; BUF_SIZE] = [0; BUF_SIZE];

  let raw_chunk = chunk.as_ptr() as *mut usize;

  let start = PreciseTime::now();

  unsafe {
    for i in 0..chunk.len() {
      ptr::write(raw_chunk.offset(i as isize), i);
    }
  }

  let duration_write = start.to(PreciseTime::now());

  let mut sum = 0;

  let start_read = PreciseTime::now();

  for i in 0..chunk.len() {
    sum = sum + chunk[i] % 2;
  }

  let duration_read = start_read.to(PreciseTime::now());
  
  return (duration_write, duration_read, sum * 2 * mem::size_of::<usize>());
}