use crate::common;
use std::fs::File;
use std::io::{self, BufRead};
use simple_error::{SimpleError, bail};
use std::error::Error;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
enum Instruction {
    North(i64),
    East(i64),
    West(i64),
    South(i64),
    Left(i64),
    Right(i64),
    Forward(i64)
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num = s[1..].parse::<i64>()?;
        match s.chars().next().ok_or_else(|| SimpleError::new(""))? {
            'W' => Ok(Instruction::West(num)),
            'N' => Ok(Instruction::North(num)),
            'E' => Ok(Instruction::East(num)),
            'S' => Ok(Instruction::South(num)),
            'F' => Ok(Instruction::Forward(num)),
            'L' => Ok(Instruction::Left(num)),
            'R' => Ok(Instruction::Right(num)),
            _ => bail!("no such instruction")
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct State {
    east: i64,
    north: i64,
    direction: i64,
    waypoint_east: i64,
    waypoint_north: i64
}

impl State {
    fn new(east: i64, north: i64, direction: i64, waypoint_east: i64, waypoint_north: i64) -> State {
        State { east, north, direction, waypoint_east, waypoint_north }
    }

    fn next(&self, instruction: Instruction) -> common::BoxResult<State> {
        match instruction {
            Instruction::North(i) => Ok(State::new(self.east, self.north + i, self.direction, self.waypoint_east, self.waypoint_north)),
            Instruction::East(i) =>  Ok(State::new(self.east + i, self.north, self.direction, self.waypoint_east, self.waypoint_north)),
            Instruction::South(i) => self.next(Instruction::North(-1 * i)),
            Instruction::West(i) =>  self.next(Instruction::East(-1 * i)),
            Instruction::Right(i) => Ok(State::new(self.east, self.north, (self.direction + i).rem_euclid(360), self.waypoint_east, self.waypoint_north)),
            Instruction::Left(i) =>  self.next(Instruction::Right(-1 * i)),
            Instruction::Forward(i) => {
                match self.direction {
                    0 => self.next(Instruction::North(i)),
                    90 => self.next(Instruction::East(i)),
                    180 => self.next(Instruction::South(i)),
                    270 => self.next(Instruction::West(i)),
                    _ => bail!(format!("invalid direction: {}", self.direction))
                }
            }
        }
    }

    fn next_actual(&self, instruction: Instruction) -> common::BoxResult<State> {
        match instruction {
            Instruction::North(i) => Ok(State::new(self.east, self.north, self.direction, self.waypoint_east, self.waypoint_north + i)),
            Instruction::East(i) =>  Ok(State::new(self.east, self.north, self.direction, self.waypoint_east + i, self.waypoint_north)),
            Instruction::South(i) => self.next(Instruction::North(-1 * i)),
            Instruction::West(i) =>  self.next(Instruction::East(-1 * i)),
            Instruction::Forward(i) => Ok(State::new(self.east + self.waypoint_east * i, self.north + self.waypoint_north * i, self.direction, self.waypoint_east, self.waypoint_north)),
            Instruction::Right(i) => {
                match i.rem_euclid(360) {
                    0 => Ok(*self),
                    90 => Ok(State::new(self.east, self.north, self.direction, self.waypoint_north, -1 * self.waypoint_east)),
                    180 => Ok(State::new(self.east, self.north, self.direction, -1 * self.waypoint_east, -1 * self.waypoint_north)),
                    270 => Ok(State::new(self.east, self.north, self.direction, -1 * self.waypoint_north, self.waypoint_east)),
                    _ => bail!(format!("invalid direction: {}", self.direction))
                }
            }
            Instruction::Left(i) =>  self.next(Instruction::Right(-1 * i)),
        }
    }

    fn manhattan(&self) -> i64 {
        self.east.abs() + self.north.abs()
    }

    fn process(&self, instructions: &Vec<Instruction>, next_fn: fn(&State, Instruction) -> common::BoxResult<State>) -> common::BoxResult<State> {
        println!("processing: \n\n{:?}", self);
        let mut state = *self;
        for instruction in instructions {
            state = next_fn(&state, *instruction)?;
            println!("{:?}", state);
        }
        Ok(state)
    }
}

pub fn answer() -> common::BoxResult<(i64, i64)> {
    let input = read_input("day12_input")?;
    let start = State::new(0, 0, 90, 10, 1);
    let end_first = start.process(&input, State::next)?;
    let end_actual = start.process(&input, State::next_actual)?;
    Ok((end_first.manhattan(), end_actual.manhattan()))
}

fn read_input(file: &str) -> common::BoxResult<Vec<Instruction>> {
    let file = File::open(file)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map(|l| -> common::BoxResult<_> { Ok(l?.parse::<Instruction>()?) }).collect::<Result<_, _>>()?)
}