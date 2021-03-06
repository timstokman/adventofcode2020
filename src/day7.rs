use crate::common;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::vec::Vec;
use simple_error::SimpleError;

pub fn answer() -> common::BoxResult<(usize, i32)> {
    let input = read_input("day7_input")?;
    Ok((count_bags_can_hold("shiny gold", &input), count_bags_necessary("shiny gold", &input)))
}

fn count_bags_necessary(target_color: &str, bag_tree: &HashMap<String, Vec<(String, i32)>>) -> i32 {
    let mut table = HashMap::<String, i32>::new();
    fill_table_necessary(target_color, &mut table, bag_tree);
    table[target_color]
}

fn fill_table_necessary(color: &str, necessary_table: &mut HashMap<String, i32>, bag_tree: &HashMap<String, Vec<(String, i32)>>) {
    let mut total = 0;
    for (inner_color, cnt) in &bag_tree[color] {
        fill_table_necessary(inner_color, necessary_table, bag_tree);
        let num_bags = (necessary_table[inner_color] + 1) * cnt;
        total += num_bags;
    }
    necessary_table.insert(color.to_owned(), total);
}

fn count_bags_can_hold(target_color: &str, bag_tree: &HashMap<String, Vec<(String, i32)>>) -> usize {
    let mut table = HashMap::<String, bool>::new();
    for color in bag_tree.keys() {
        fill_table_can_hold(&color, &mut table, bag_tree, target_color);
    }
    table.iter()
         .filter(|(_, can_contain)| **can_contain)
         .count()
}

fn fill_table_can_hold(color: &str, can_hold_table: &mut HashMap<String, bool>, bag_tree: &HashMap<String, Vec<(String, i32)>>, target_color: &str) {
    if can_hold_table.contains_key(color) {
        return;
    }
    can_hold_table.insert(color.to_owned(), false);
    for (inner_color, _) in &bag_tree[color] {
        if inner_color == target_color {
            can_hold_table.insert(color.to_owned(), true);
            break;
        } else {
            fill_table_can_hold(&inner_color, can_hold_table, bag_tree, target_color);
            if can_hold_table[inner_color] {
                can_hold_table.insert(color.to_owned(), true);
                break;
            }
        }
    }
}

fn read_input(file: &str) -> common::BoxResult<HashMap<String, Vec<(String, i32)>>> {
    let file = File::open(file)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|line| -> common::BoxResult<(String, Vec<(String, i32)>)> {
        let read_line = line?;
        let split_rule = read_line.split(" bags contain ").collect::<Vec<_>>();
        let result_bags = 
            split_rule[1].split(", ")
                         .filter(|b| *b != "no other bags.")
                         .map(|b| -> common::BoxResult<(String, i32)> {
                             let bag_spec = &b[0..b.find(" bag").ok_or_else(|| SimpleError::new("bag not found"))?];
                             Ok((bag_spec[2..].to_string(), b[0..1].parse::<i32>()?))
                         })
                         .collect::<Result<_, _>>()?;
        Ok((split_rule[0].to_string(), result_bags))
    }).collect::<Result<_, _>>()?)
}