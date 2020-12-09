use crate::common;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;
use simple_error::bail;

pub fn answer() -> common::BoxResult<(i64, i64)> {
    let input = read_input("day9_input")?;
    let first_not_sum = first_num_not_sum(&input)?;
    let encryption_weakness = find_encryption_weakness(&input, first_not_sum)?;
    Ok((first_not_sum, encryption_weakness))
}

fn first_num_not_sum(numbers: &[i64]) -> common::BoxResult<i64> {
    const PREAMBLE_LEN: usize = 25;
    let mut numbers_before = HashSet::with_capacity(PREAMBLE_LEN);
    for (i, num) in numbers.iter().enumerate() {
        if i >= PREAMBLE_LEN {
            let has_sum = numbers_before.iter().any(|num_check| numbers_before.contains(&(*num - num_check)));
            if !has_sum {
                return Ok(*num);
            }
            numbers_before.remove(&numbers[i - PREAMBLE_LEN]);
        }
        numbers_before.insert(*num);
    }
    bail!("no answer found")
}

fn find_encryption_weakness(numbers: &[i64], target: i64) -> common::BoxResult<i64> {
    let (mut start, mut end, mut sum) = (0, 0, 0);

    fn get_weakness(range: &[i64]) -> i64 {
        range.iter().min().expect("no numbers") + range.iter().max().expect("no numbers")
    }

    for num in numbers.iter() {
        sum += num;
        end += 1;
        while sum > target {
            sum -= numbers[start];
            start += 1;
        }
        if sum == target {
            return Ok(get_weakness(&numbers[start..end]));
        }
    }
    bail!("no answer found")
}

fn read_input(file: &str) -> common::BoxResult<Vec<i64>> {
    let file = File::open(file)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|l| -> common::BoxResult<_> { Ok(l?.parse::<i64>()?) }).collect::<Result<_, _>>()?)
}