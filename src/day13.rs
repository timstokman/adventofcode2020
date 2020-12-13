use crate::common;
use std::fs::File;
use std::io::{self, BufRead};
use simple_error::SimpleError;

fn wait_time(start_time: i64, bus_nr: i64) -> i64 {
    bus_nr - (start_time % bus_nr)
}

fn chinese_remainder(a: &[i64], n: &[i64]) -> Option<i64> {
    let prod: i64 = n.iter().product();
    let sum: i64 = 
        a.iter()
        .zip(n.iter())
        .map(|(&a_i, &n_i)| {
            let p = prod / n_i;
            mod_inv(p, n_i).map(|inv| a_i * p * inv)
        })
        .collect::<Option<Vec<i64>>>()?
        .iter()
        .sum();
    return Some(sum % prod);
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}
 
fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn part_1(start_time: i64, bus_nrs: &[Option<i64>]) -> common::BoxResult<i64> {
    let answers = bus_nrs.iter().filter_map(|b| b.map(|bus_nr| (bus_nr, wait_time(start_time, bus_nr)))).collect::<Vec<_>>();
    let min_wait = answers.iter().map(|a| a.1).min().ok_or_else(|| SimpleError::new("no bus nrs"))?;
    let answer = answers.iter().find(|a| a.1 == min_wait).ok_or_else(|| SimpleError::new("no bus nrs"))?;
    Ok(answer.0 * answer.1)
}

fn part_2(bus_nrs: &[Option<i64>]) -> common::BoxResult<i64> {
    let a = bus_nrs.iter().enumerate().filter_map(|(i, bus_nr)| bus_nr.map(|b| b - i as i64)).collect::<Vec<_>>();
    let n = bus_nrs.iter().filter_map(|bus_nr| *bus_nr).collect::<Vec<_>>();
    Ok(chinese_remainder(&a, &n).ok_or_else(|| SimpleError::new("no solution"))?)
}

pub fn answer() -> common::BoxResult<(i64, i64)> {
    let input = read_input("day13_input")?;
    Ok((part_1(input.0, &input.1)?, part_2(&input.1)?))
}

fn read_input(file: &str) -> common::BoxResult<(i64, Vec<Option<i64>>)> {
    let file = File::open(file)?;
    let reader = io::BufReader::new(file);
    let mut lines_it = reader.lines();
    let start_time = lines_it.next().ok_or_else(|| SimpleError::new("no start time"))??.parse::<i64>()?;
    let dep_times = lines_it.next().ok_or_else(|| SimpleError::new("no bus nrs"))??.split(',').map(|i| -> common::BoxResult<Option<i64>> { if i == "x" { Ok(None) } else { Ok(Some(i.parse::<i64>()?)) } }).collect::<Result<_, _>>()?;
    Ok((start_time, dep_times))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column() {
        assert_eq!(chinese_remainder(&[2, 3, 2], &[3, 5, 7]), Some(23));
    }
}