use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;
use regex::Regex;
use crate::common;
use simple_error::SimpleError;

struct PasswordEntry {
    first_num: usize,
    second_num: usize,
    policy_letter: char,
    password: Vec<char>
}

impl PasswordEntry {
    fn is_valid_old(&self) -> bool {
        let num = self.password.iter()
                               .filter(|c| **c == self.policy_letter)
                               .count();
        num >= self.first_num && num <= self.second_num
    }

    fn is_valid_new(&self) -> bool {
        let first_pos = self.password[self.first_num - 1];
        let second_pos = self.password[self.second_num - 1];
        (first_pos == self.policy_letter) ^ (second_pos == self.policy_letter)
    }
}

pub fn answer() -> common::BoxResult<(usize, usize)> {
    let input = read_input("day2_input")?;
    Ok((num_valid_old_password_entries(&input), num_valid_new_password_entries(&input)))
}

fn num_valid_old_password_entries(entries: &[PasswordEntry]) -> usize {
    entries.iter().filter(|p| p.is_valid_old()).count()
}

fn num_valid_new_password_entries(entries: &[PasswordEntry]) -> usize {
    entries.iter().filter(|p| p.is_valid_new()).count()
}

fn read_input(file: &str) -> common::BoxResult<Vec<PasswordEntry>> {
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)")?;
    let file = File::open(file)?;
    let reader = io::BufReader::new(file);
    let mut result = Vec::<PasswordEntry>::new();
    for line in reader.lines() {
        let line_found = line?;
        let captures = re.captures(&line_found).ok_or_else(|| SimpleError::new("could not parse password entry"))?;
        result.push(
            PasswordEntry { 
                first_num: captures[1].parse::<usize>()?,
                second_num: captures[2].parse::<usize>()?, 
                policy_letter: captures[3].chars().next().ok_or_else(|| SimpleError::new("could not find policy_letter"))?, 
                password: captures[4].to_string().chars().collect()
            }
        )
    }
    Ok(result)
}