use crate::common;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

pub fn answer() -> common::BoxResult<(usize, usize)> {
    let input = read_input("day6_input")?;
    let sum_any_answer_yes = input.iter().map(|g| any_answer_yes(&g)).sum();
    let sum_all_answer_yes = input.iter().map(|g| all_answer_yes(&g)).sum();
    Ok((sum_any_answer_yes, sum_all_answer_yes))
}

fn any_answer_yes(group: &[String]) -> usize {
    group.iter().flat_map(|m| m.chars()).collect::<HashSet<char>>().len()
}

fn all_answer_yes(group: &[String]) -> usize {
    let mut sets = group.iter()
                        .map(|m| m.chars().collect::<HashSet<char>>());
    let first = sets.next().expect("should be at least one line in a group");
    sets.fold(first, |l, r| &l & &r)
        .len()
}

fn read_input(file: &str) -> common::BoxResult<Vec<Vec<String>>> {
    let file = File::open(file)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines.split(|line| line.is_empty()).map(|g| g.to_vec()).collect())
}