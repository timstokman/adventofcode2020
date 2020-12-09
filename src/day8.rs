use crate::common;
use std::fs::File;
use std::io::{self, BufRead};
use simple_error::SimpleError;

#[derive(PartialEq, Clone, Copy)]
enum Instruction {
    Acc,
    Jmp,
    Nop,
}

pub fn answer() -> common::BoxResult<(i64, i64)> {
    let mut input = read_input("day8_input")?;
    Ok((execute(&input).0, find_flipped_instruction(&mut input).0))
}

fn moved_position(pos: usize, diff: i64) -> usize {
    if diff > 0 {
        pos + diff as usize
    } else {
        pos - ((-diff) as usize)
    }
}

fn find_flipped_instruction(instructions: &mut Vec<(Instruction, i64)>) -> (i64, usize, Option<usize>) {
    let mut table: Vec<Option<bool>> = (0..(instructions.len() + 1)).map(|_| None).collect();
    let mut accumulator = 0;
    let mut position = 0;
    let mut flipped_instruction: Option<usize> = None;

    while position != instructions.len() {
        let mut instruction = instructions[position];
        if flipped_instruction == None && instruction.0 != Instruction::Acc {
            let (alt_move, alt_instruction) = match instruction {
                (Instruction::Nop, i) => (moved_position(position, i), Instruction::Jmp),
                (Instruction::Jmp, _) => (position + 1, Instruction::Nop),
                _ => unreachable!()
            };
            fill_flipped_table(alt_move, &mut table, instructions);
            if table[alt_move] == Some(true) {
                instruction.0 = alt_instruction;
                flipped_instruction = Some(position);
            }
        }
        match instruction {
            (Instruction::Acc, i) => {
                accumulator += i;
                position += 1;
            },
            (Instruction::Nop, _) => {
                position += 1;
            },
            (Instruction::Jmp, i) => {
                position = moved_position(position, i);
            }
        }
    }

    (accumulator, position, flipped_instruction)
}

fn fill_flipped_table(pos: usize, table: &mut Vec<Option<bool>>, instructions: &Vec<(Instruction, i64)>) {
    if pos >= instructions.len() {
        table[pos] = Some(true);
        return;
    }
    else if table[pos] != None {
        return;
    }
    let next_pos = match instructions[pos] {
        (Instruction::Acc, _) => pos + 1,
        (Instruction::Nop, _) => pos + 1,
        (Instruction::Jmp, i) => {
            moved_position(pos, i)
        }
    };
    table[pos] = Some(false);
    fill_flipped_table(next_pos, table, instructions);
    table[pos] = table[next_pos];
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
                position = moved_position(position, i);
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