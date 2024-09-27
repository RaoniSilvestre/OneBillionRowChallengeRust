use onebrc::*;
use std::fs::*;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use rustc_hash::FxHashMap;

fn main() -> Result<(), std::io::Error> {
    let start = Instant::now();

    let mut values: FxHashMap<Box<[u8]>, Data> = FxHashMap::default();

    let file = File::open("./data/measurements.txt")?;
    let buffer = BufReader::new(file);
    let buffer_lines = buffer.lines();

    for line_result in buffer_lines {
        evaluate_line_result(&mut values, line_result)
    }

    let duration = start.elapsed();

    for (k, v) in values {
        let (a, b, c, d) = (
            (v.min as f64) / 10.0,
            (v.total as f64) / 10.0,
            v.counter as f64,
            (v.max as f64) / 10.0,
        );
        //println!(
        //    "{} : {}/{}/{}",
        //    String::from_utf8(k.to_vec()).unwrap(),
        //    a,
        //    b / c,
        //    d
        //)
    }

    println!("Duration: {:?}", duration);

    for _ in 0..100000 {
        get_val(b"Ohio;49.3").unwrap();
    }
    Ok(())
}
