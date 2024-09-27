use core::{panic, str};
use std::{
    cmp::{max, min},
    io::Error,
};

use memchr::memchr;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Copy)]
pub struct Data {
    pub min: i64,
    pub max: i64,
    pub total: i64,
    pub counter: i64,
}

impl Data {
    pub fn new(value: i64) -> Data {
        Data {
            min: value,
            max: value,
            total: value,
            counter: 1,
        }
    }

    pub fn process(&mut self, value: i64) {
        if value < self.min {
            self.min = value
        }

        if value > self.max {
            self.max = value
        }

        self.total += value;
        self.counter += 1;
    }

    pub fn add_another(&mut self, value: Data) {
        self.min = min(self.min, value.min);
        self.max = max(self.max, value.max);
        self.total += value.total;
        self.counter += value.counter;
    }
}

impl Default for Data {
    fn default() -> Self {
        Data {
            min: 1000000,
            max: -100000,
            total: 0,
            counter: 0,
        }
    }
}

pub fn evaluate_line_result(
    values: &mut FxHashMap<Box<[u8]>, Data>,
    line_result: Result<String, Error>,
) {
    match line_result {
        Ok(line) => calculate_line(values, line.as_bytes()),
        Err(_) => (),
    }
}

pub fn calculate_line(values: &mut FxHashMap<Box<[u8]>, Data>, line: &[u8]) {
    match get_val(&line) {
        Ok((station_name, station_data)) => add_to_hashmap(values, (station_name, station_data)),
        Err(_) => (),
    }
}

pub fn add_to_hashmap(values: &mut FxHashMap<Box<[u8]>, Data>, station: (&[u8], i64)) {
    match values.contains_key(station.0) {
        false => add_new_value(values, (station.0, station.1)),
        true => update_value(values, (station.0, station.1)),
    }
}

pub fn add_new_value(
    values: &mut FxHashMap<Box<[u8]>, Data>,
    (station_name, station_data): (&[u8], i64),
) {
    values.insert(station_name.into(), Data::new(station_data));
}

pub fn update_value(
    values: &mut FxHashMap<Box<[u8]>, Data>,
    (station_name, station_data): (&[u8], i64),
) {
    match values.get_mut(station_name) {
        Some(data) => data.process(station_data),
        None => panic!("Isso é literalmente impossível"),
    }
}

pub fn get_val(line: &[u8]) -> Result<(&[u8], i64), ()> {
    match memchr(b';', line) {
        None => Err(()),
        Some(comma_separator) => {
            let name = &line[..comma_separator];
            let value = &line[comma_separator + 1..];

            match memchr(b'.', value) {
                Some(dot_separator) => {
                    let valuable_value = &value[..dot_separator];
                    let not_so_valuable = &value[dot_separator + 1..];
                    let num = [valuable_value, not_so_valuable].concat();

                    return Ok((name, str::from_utf8(&num).unwrap().parse::<i64>().unwrap()));
                }
                None => panic!("Fudeu"),
            }
        }
    }
}
