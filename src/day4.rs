use crate::common;
use std::vec::Vec;
use std::collections::HashMap;
use std::fs::File;
use regex::Regex;
use std::io::{self, BufRead};

static PASSPORT_KEYS: [&str; 7] = [
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "pid",
];

pub fn answer() -> common::BoxResult<(usize, usize)> {
    let input = read_input("day4_input")?;
    Ok((input.iter().filter(|p| is_valid_passport_simple(&p)).count(), 
        input.iter().filter(|p| is_valid_passport(&p)).count()))
}

fn is_valid_passport_simple(passport: &HashMap<String, String>) -> bool {
    PASSPORT_KEYS.iter().all(|k| passport.contains_key(*k))
}

fn is_valid_passport(passport: &HashMap<String, String>) -> bool {
    is_valid_passport_simple(passport) &&
    PASSPORT_KEYS.iter().all(|key| is_valid_passport_entry(&key, &passport.get::<str>(key).expect("key not found")))
}

fn is_valid_passport_entry(key: &str, value: &str) -> bool {
    match key {
        "byr" => (1920..=2002).contains(&value.parse::<i64>().unwrap_or(-1)),
        "iyr" => (2010..=2020).contains(&value.parse::<i64>().unwrap_or(-1)),
        "eyr" => (2020..=2030).contains(&value.parse::<i64>().unwrap_or(-1)),
        "hgt" => {
            let re = Regex::new(r"^([0-9]+)(cm|in)$").expect("should be a valid regex");
            match re.captures(value) {
                Some(captures) => {
                    let num = captures[1].parse::<i64>().expect("should be a number at this point");
                    if &captures[2] == "cm" {
                        (150..=193).contains(&num)
                    } else {
                        (59..=76).contains(&num)
                    }
                }, _ => false
            }
        },
        "hcl" => Regex::new(r"^#[0-9a-f]+$").expect("should be a valid regex").is_match(value),
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().any(|cont| *cont == value),
        "pid" => Regex::new(r"^[0-9]{9}$").expect("should be a valid regex").is_match(value),
        _ => false
    }
}

fn read_input(file: &str) -> common::BoxResult<Vec<HashMap<String, String>>> {
    let file = File::open(file)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines.split(|line| line.is_empty()).map(read_passport).collect())
}

fn read_passport(lines: &[String]) -> HashMap<String, String> {
    lines.iter()
         .flat_map(|l| l.split(' '))
         .map(|passport_entry| passport_entry.split(':').collect::<Vec<&str>>())
         .map(|passport_entry| (passport_entry[0].to_string(), passport_entry[1].to_string()))
         .collect()
}