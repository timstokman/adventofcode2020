use crate::common;
use std::fs::File;
use std::io::{self, BufRead};
use simple_error::SimpleError;

enum Instruction {
    Acc,
    Jmp,
    Nop,
}

pub fn answer() -> common::BoxResult<i64> {
    let input = read_input("day8_input")?;
    Ok(execute(&input).0)
}

fn execute(instructions: &Vec<(Instruction, i64)>) -> (i64, usize) {
    let mut accumulator = 0;
    let mut position = 0;
    let mut already_executed = vec![false; instructions.len()];

    while !already_executed[position] && position != instructions.len() {
        already_executed[position] = true;
        match instructions[position] {
            (Instruction::Acc, i) => {
                accumulator += i;
                position += 1;
            },
            (Instruction::Nop, _) => {
                position += 1;
            },
            (Instruction::Jmp, i) => {
                if i > 0 {
                    position += i as usize;
                } else {
                    position -= (-i) as usize;
                }
            }
        }
    }

    (accumulator, position)
}

fn read_input(file: &str) -> common::BoxResult<Vec<(Instruction, i64)>> {
    let file = File::open(file).expect("error opening file");
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|l| -> common::BoxResult<(Instruction, i64)> {
        let line = l?;
        let num = line[4..].parse::<i64>()?;
        match &line[0..3] {
            "acc" => Ok((Instruction::Acc, num)),
            "jmp" => Ok((Instruction::Jmp, num)),
            "nop" => Ok((Instruction::Nop, num)),
            _ => Err(Box::new(SimpleError::new("unknown instruction")))
        }
    }).collect::<Result<_, _>>()?)
}