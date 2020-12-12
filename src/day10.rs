use crate::common;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter;
use std::cmp;
use simple_error::{bail, SimpleError};

const MIN_DIFFERENCE: i64 = 1;
const AVG_DIFFERENCE: i64 = 2;
const MAX_DIFFERENCE: i64 = 3;

pub fn answer() -> common::BoxResult<(i64, i64)> {
    let input = read_input("day10_input")?;
    let difference_steps = count_difference_steps(&input)?;
    let different_arrangements = count_different_arrangements(&input);
    Ok((difference_steps.0 * difference_steps.1, different_arrangements))
}

fn count_different_arrangements(adapters: &Vec<i64>) -> i64 {
    let mut result = vec![1; adapters.len()];
    for (i, adapter) in adapters.iter().enumerate().rev().skip(1) {
        let possible_jumps = ((i + 1)..cmp::min(i + 1 + MAX_DIFFERENCE as usize, adapters.len()))
                                 .filter(|next_i| adapters[*next_i] - adapter <= MAX_DIFFERENCE);
        result[i] = possible_jumps.map(|next_i| result[next_i])
                                  .sum();
    }
    result[0]
}

fn count_difference_steps(adapters: &Vec<i64>) -> common::BoxResult<(i64, i64)> {
    let mut adapters_iter = adapters.iter().cloned();
    let mut current = adapters_iter.next().ok_or_else(|| SimpleError::new("no values found"))?;
    let mut difference_count: (i64, i64) = (0, 0);
    for adapter in adapters_iter {
        let diff = adapter - current;
        match diff {
            MIN_DIFFERENCE => { difference_count.0 += 1; },
            AVG_DIFFERENCE => { },
            MAX_DIFFERENCE => { difference_count.1 += 1; },
            _ => bail!("no chain found")
        }
        current = adapter;
    }
    Ok(difference_count)
}

fn read_input(file: &str) -> common::BoxResult<Vec<i64>> {
    let file = File::open(file)?;
    let reader = io::BufReader::new(file);
    let mut adapters: Vec<i64> = reader.lines().map(|l| -> common::BoxResult<_> { Ok(l?.parse::<i64>()?) }).collect::<Result<_, _>>()?;
    adapters.sort();
    let last_value = adapters.last().ok_or_else(|| SimpleError::new("no values found"))? + 3;
    Ok(iter::once(0i64).chain(adapters.iter().cloned()).chain(iter::once(last_value)).collect())
}