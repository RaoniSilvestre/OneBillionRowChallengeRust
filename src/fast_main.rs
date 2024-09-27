use onebrc::*;
use rustc_hash::FxHashMap;
use std::fs::*;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::thread;
use std::time::Instant;

static TOTAL_LINES: u64 = 1000000000;
static NUMBER_OF_THREADS: u64 = 12;

fn main() -> Result<(), std::io::Error> {
    let start = Instant::now();
    let mut handles = vec![];

    let lines_per_thread = TOTAL_LINES / NUMBER_OF_THREADS;

    println!("{} - {}", TOTAL_LINES, lines_per_thread);

    for i in 0..NUMBER_OF_THREADS {
        let handle = thread::Builder::new().name(format!("{i}")).spawn(move || {
            let val_inicial = lines_per_thread * i;
            let val_final = lines_per_thread * (i + 1);

            let mut values: FxHashMap<Box<[u8]>, Data> = FxHashMap::default();

            let file = File::open("./data/measurements.txt").unwrap();
            let mut buffer = BufReader::new(file);
            buffer.seek(SeekFrom::Start(val_inicial as u64)).unwrap();
            let mut lines = buffer.lines();

            for _ in val_inicial..val_final {
                evaluate_line_result(&mut values, lines.next().unwrap())
            }

            values
        });
        handles.push(handle);
    }

    let mut final_values: FxHashMap<Box<[u8]>, Data> = FxHashMap::default();

    for handle in handles {
        let val = handle?.join().unwrap();
        for (k, v) in val {
            final_values
                .entry(k)
                .and_modify(|e| {
                    e.add_another(v);
                })
                .or_insert(v);
        }
    }

    let duration = start.elapsed();

    for (k, v) in final_values {
        println!(
            "{} : {}/{}/{}",
            String::from_utf8(k.to_vec()).unwrap(),
            to_f64(v.min),
            format!("{:.1}", to_f64(v.total) / (to_f64(v.counter) * 10.0)),
            to_f64(v.max)
        )
    }
    println!("Duration: {:?}", duration);

    Ok(())
}

fn to_f64(val: i64) -> f64 {
    (val as f64) / 10.0
}
