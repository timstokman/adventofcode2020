use crate::common;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;
use simple_error::SimpleError;

pub fn answer() -> common::BoxResult<(usize, usize)> {
    let seat_ids = get_seat_ids("day5_input")?;
    let max = seat_ids.iter().max().ok_or_else(|| SimpleError::new("no seat ids"))?;
    let min = seat_ids.iter().min().ok_or_else(|| SimpleError::new("no seat ids"))?;
    let sum = seat_ids.iter().sum();
    Ok((*max, calculate_missing_seat(seat_ids.len(), sum, *min)))
}

fn calculate_missing_seat(len: usize, sum: usize, min: usize) -> usize {
    // SUM(1..n) = n * (n + 1) / 2
    // So if we sum all the seat numbers, substract the bottom part, subtract the actual sum, then the seat number remains
    let below_missing_sum = (min - 1) * min / 2;
    let expected_sum = (len + min) * (len + min + 1) / 2;
    expected_sum - below_missing_sum - sum
}

fn get_seat_ids(file: &str) -> common::BoxResult<Vec<usize>> {
    let file = File::open(file)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String>  = reader.lines()
                                    .collect::<Result<_, _>>()?;
    Ok(lines.iter().map(|l| get_seat_id(&l)).collect())
}

fn get_seat_id(code: &str) -> usize {
    8 * get_row(&code[0..7]) + get_column(&code[7..10])
}

fn get_row(code: &str) -> usize {
    code.chars()
        .enumerate()
        .map(|(i, c)| ((c == 'B') as usize) << (code.len() - i - 1))
        .sum()
}

fn get_column(code: &str) -> usize {
    code.chars()
        .enumerate()
        .map(|(i, c)| ((c == 'R') as usize) << i)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column() {
        assert_eq!(get_column("RLR"), 5);
    }

    #[test]
    fn test_row() {
        assert_eq!(get_row("FBFBBFF"), 44);
    }

    #[test]
    fn test_seat_id() {
        assert_eq!(get_seat_id("FBFBBFFRLR"), 357);
    }

    #[test]
    fn test_missing_seat() {
        assert_eq!(calculate_missing_seat(3, 2, 11), 48);
    }
}