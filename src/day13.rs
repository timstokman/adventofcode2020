use crate::common;
use std::fs::File;
use std::io::{self, BufRead};
use simple_error::SimpleError;

fn wait_time(start_time: i64, bus_nr: i64) -> i64 {
    bus_nr - (start_time % bus_nr)
}

fn part_1(start_time: i64, bus_nrs: &[i64]) -> common::BoxResult<i64> {
    let answers = bus_nrs.iter().map(|bus_nr| (*bus_nr, wait_time(start_time, *bus_nr))).collect::<Vec<_>>();
    let min_wait = answers.iter().map(|a| a.1).min().ok_or_else(|| SimpleError::new("no bus nrs"))?;
    let answer = answers.iter().find(|a| a.1 == min_wait).ok_or_else(|| SimpleError::new("no bus nrs"))?;
    Ok(answer.0 * answer.1)
}

pub fn answer() -> common::BoxResult<i64> {
    let input = read_input("day13_input")?;
    Ok(part_1(input.0, &input.1)?)
}

fn read_input(file: &str) -> common::BoxResult<(i64, Vec<i64>)> {
    let file = File::open(file)?;
    let reader = io::BufReader::new(file);
    let mut lines_it = reader.lines();
    let start_time = lines_it.next().ok_or_else(|| SimpleError::new("no start time"))??.parse::<i64>()?;
    let dep_times = lines_it.next().ok_or_else(|| SimpleError::new("no bus nrs"))??.split(',').filter(|i| *i != "x").map(|i| -> common::BoxResult<i64> { Ok(i.parse::<i64>()?) }).collect::<Result<_, _>>()?;
    Ok((start_time, dep_times))
}