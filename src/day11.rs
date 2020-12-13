use crate::common;
use std::fs::File;
use std::io::{self, BufRead};
use std::fmt;
use std::fmt::Write;
use simple_error::bail;

#[derive(PartialEq, Clone, Copy)]
enum SeatState {
    Floor,
    Empty,
    Occupied
}

impl fmt::Debug for SeatState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match *self {
            SeatState::Floor => '.',
            SeatState::Occupied => '#',
            SeatState::Empty => 'L'
        };
        f.write_char(c)
    }
}

impl SeatState {
    fn next_1(self, number_occupied: usize) -> SeatState {
        match (self, number_occupied == 0, number_occupied >= 4) {
            (SeatState::Empty, true, false) => SeatState::Occupied,
            (SeatState::Occupied, false, true) => SeatState::Empty,
            _ => self
        }
    }

    fn next_2(self, number_occupied: usize) -> SeatState {
        match (self, number_occupied == 0, number_occupied >= 5) {
            (SeatState::Empty, true, false) => SeatState::Occupied,
            (SeatState::Occupied, false, true) => SeatState::Empty,
            _ => self
        }
    }
}

#[derive(PartialEq, Clone)]
struct SeatStates {
    pub states: Vec<Vec<SeatState>>
}

static NEIGHBOURS: [(i64, i64); 8] = [(-1i64, 0i64), (1i64, 0i64), (0i64, -1i64), (0i64, 1i64), (-1i64, -1i64), (1i64, 1i64), (-1i64, 1i64), (1i64, -1i64)];

fn add(i: usize, d: i64) -> usize {
    if d.is_negative() {
        i.wrapping_sub(d.wrapping_abs() as usize)
    } else {
        i + d as usize
    }
}

impl fmt::Debug for SeatStates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            &self.states
                 .iter()
                 .map(|state_line|
                    state_line.iter()
                              .map(|state| format!("{:?}", state))
                              .collect::<Vec<_>>()
                              .join(""))
                 .collect::<Vec<_>>()
                 .join("\n"))
    }
}

impl SeatStates {
    fn new(states: Vec<Vec<SeatState>>) -> SeatStates {
        SeatStates {
            states
        }
    }

    fn count_steady_state_occupied(self, next_fn: fn(&SeatStates, usize, usize) -> SeatState) -> usize {
        let mut state = self;
        let mut has_changed = true;
        while has_changed {
            let next_state = state.next_state(next_fn);
            has_changed = next_state != state;
            state = next_state;
        }
        state.num_states(SeatState::Occupied)
    }

    fn num_states(&self, state: SeatState) -> usize {
        self.states.iter().flat_map(|state_line| state_line.iter().filter(|s| **s == state)).count()
    }

    fn get_state(&self, x: usize, y: usize) -> Option<SeatState> {
        self.states.get(y).and_then(|gotten_state_line| gotten_state_line.get(x).map(|s|*s))
    }

    fn num_neighbours_occupied(&self, x: usize, y: usize) -> usize {
        NEIGHBOURS.iter()
                  .cloned()
                  .filter_map(|(n_x, n_y)| self.get_state(add(x, n_x), add(y, n_y)))
                  .filter(|s| *s == SeatState::Occupied)
                  .count()
    }

    fn num_neighbours_occupied_sight(&self, x: usize, y: usize) -> usize {
        NEIGHBOURS.iter()
                  .cloned()
                  .map(|(n_x, n_y)| 
                    (1..).map(|mult| self.get_state(add(x, mult * n_x), add(y, mult * n_y)))
                         .take_while(|s| *s != None)
                         .find(|s| *s != Some(SeatState::Floor)) == Some(Some(SeatState::Occupied)))
                  .filter(|s| *s == true)
                  .count()
    }

    fn next_1(&self, x: usize, y: usize) -> SeatState {
        self.states[y][x].next_1(self.num_neighbours_occupied(x, y))
    }

    fn next_2(&self, x: usize, y: usize) -> SeatState {
        self.states[y][x].next_2(self.num_neighbours_occupied_sight(x, y))
    }

    fn next_state(&self, next_fn: fn(&SeatStates, usize, usize) -> SeatState) -> SeatStates {
        SeatStates::new(
            self.states
                .iter()
                .enumerate()
                .map(|(y, state_line)| {
                    state_line.iter()
                              .enumerate()
                              .map(|(x, _)| next_fn(&self, x, y))
                              .collect()
                })
                .collect())
    }

    fn read_input(file: &str) -> common::BoxResult<SeatStates> {
        let file = File::open(file)?;
        let reader = io::BufReader::new(file);
        Ok(SeatStates::new(reader.lines().map(|l| -> common::BoxResult<Vec<SeatState>> {
            Ok(l?.chars()
                .map(|c| -> common::BoxResult<SeatState> {
                    match c {
                        '.' => Ok(SeatState::Floor),
                        '#' => Ok(SeatState::Occupied),
                        'L' => Ok(SeatState::Empty),
                        _ => bail!("invalid seat state")
                    }
                })
                .collect::<Result<Vec<SeatState>, _>>()?)
        }).collect::<Result<_, _>>()?))
    }
}

pub fn answer() -> common::BoxResult<(usize, usize)> {
    let input = SeatStates::read_input("day11_input")?;
    Ok((input.clone().count_steady_state_occupied(SeatStates::next_1), input.count_steady_state_occupied(SeatStates::next_2)))
}
