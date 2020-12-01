use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;
use crate::common;

const TOTAL_REPORT: i64 = 2020;

pub fn answer() -> common::Result<(i64, i64)> {
    let input = read_input()?;
    return Ok((find_pair_summing_to(&input, TOTAL_REPORT)?, find_triple_summing_to(&input, TOTAL_REPORT)?));
}

fn find_pair_summing_to(input: &HashSet<i64>, sum: i64) -> common::Result<i64> {
    for entry in input {
        let other_entry = sum - entry;
        if *entry != other_entry && input.contains(&other_entry) {
            return Ok(entry * other_entry);
        }
    }
    return Err(Box::new(common::AnswerNotFound {}))
}

fn find_triple_summing_to(input: &HashSet<i64>, sum: i64) -> common::Result<i64> {
    for entry in input {
        for entry2 in input {
            if entry != entry2 {
                let other_entry = sum - entry - entry2;
                if *entry != other_entry && *entry2 != other_entry && input.contains(&other_entry) {
                    return Ok(entry * entry2 * other_entry);
                }
            }
        }
    }
    return Err(Box::new(common::AnswerNotFound {}))
}

fn read_input() -> common::Result<HashSet<i64>> {
    let file = File::open("day1_input")?;
    let reader = io::BufReader::new(file);
    let mut result = HashSet::<i64>::new();
    for line in reader.lines() {
        result.insert(line?.parse::<i64>()?);
    }
    return Ok(result);
}