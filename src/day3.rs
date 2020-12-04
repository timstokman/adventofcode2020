use crate::common;
use std::fs::File;
use std::io::{self, BufRead};

struct RepeatingEnvironment {
    environment: Vec<Vec<bool>>,
    pub rows: usize,
    pub cols: usize
}

impl RepeatingEnvironment {
    fn new(environment: Vec<Vec<bool>>) -> RepeatingEnvironment {
        RepeatingEnvironment {
            rows: environment.len(),
            cols: environment[0].len(),
            environment: environment
        }
    }

    fn is_tree(&self, x: usize, y: usize) -> bool {
        self.environment[y][x % self.cols]
    }
}

pub fn answer() -> common::BoxResult<(i64, i64)> {
    let input = read_input("day3_input")?;
    let part1 = get_number_trees(&input, 3, 1);
    let part2 = get_number_trees(&input, 1, 1) * get_number_trees(&input, 3, 1) * get_number_trees(&input, 5, 1) * get_number_trees(&input, 7, 1) * get_number_trees(&input, 1, 2);
    Ok((part1, part2))
}

fn get_number_trees(env: &RepeatingEnvironment, x_diff: usize, y_diff: usize) -> i64 {
    let (mut x, mut y, mut num_trees) = (0, 0, 0);
    while y < env.rows {
        num_trees = num_trees + env.is_tree(x, y) as i64;
        x = x + x_diff;
        y = y + y_diff;
    }
    num_trees
}

fn read_input(file: &str) -> common::BoxResult<RepeatingEnvironment> {
    let file = File::open(file)?;
    let reader = io::BufReader::new(file);
    let mut result = Vec::<Vec::<bool>>::new();
    for line in reader.lines() {
        let line_found = line?;
        let env_row: Vec<bool> = line_found.chars().map(|c| c == '#').collect();
        result.push(env_row);
    }
    return Ok(RepeatingEnvironment::new(result))
}