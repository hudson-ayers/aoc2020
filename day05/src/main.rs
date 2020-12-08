use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;
extern crate elapsed;
use elapsed::measure_time;

fn main() {
    let filename = "input/input.txt";
    let (time, ret) = measure_time(|| solve(&filename));
    println!("part 1: {} in {}", ret, time);
    let (time2, ret2) = measure_time(|| part_2(&filename));
    println!("part 2: {} in {}", ret2, time2);
}

struct PossibleSeats {
    min_row: usize,
    num_rows: usize,
    min_column: usize,
    num_columns: usize,
}

// TODO: Pull out duplicated code between part 1 and 2
fn solve(filename: &str) -> usize {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines
        .map(|line| {
            let s = line.unwrap();
            let seat = s.chars().fold(
                PossibleSeats {
                    min_row: 0,
                    num_rows: 128,
                    min_column: 0,
                    num_columns: 8,
                },
                |mut acc, c| match c {
                    'B' => {
                        acc.num_rows /= 2;
                        acc.min_row += acc.num_rows;
                        acc
                    }
                    'F' => {
                        acc.num_rows /= 2;
                        acc
                    }
                    'R' => {
                        acc.num_columns /= 2;
                        acc.min_column += acc.num_columns;
                        acc
                    }
                    'L' => {
                        acc.num_columns /= 2;
                        acc
                    }
                    _ => {
                        panic!("Invalid input");
                    }
                },
            );
            seat.min_row * 8 + seat.min_column
        })
        .max()
        .unwrap()
}

fn part_2(filename: &str) -> usize {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut vec: Vec<usize> = lines
        .map(|line| {
            let s = line.unwrap();
            let seat = s.chars().fold(
                PossibleSeats {
                    min_row: 0,
                    num_rows: 128,
                    min_column: 0,
                    num_columns: 8,
                },
                |mut acc, c| match c {
                    'B' => {
                        acc.num_rows /= 2;
                        acc.min_row += acc.num_rows;
                        acc
                    }
                    'F' => {
                        acc.num_rows /= 2;
                        acc
                    }
                    'R' => {
                        acc.num_columns /= 2;
                        acc.min_column += acc.num_columns;
                        acc
                    }
                    'L' => {
                        acc.num_columns /= 2;
                        acc
                    }
                    _ => {
                        panic!("Invalid input");
                    }
                },
            );
            seat.min_row * 8 + seat.min_column
        })
        .collect();
    vec.sort();
    let mut last = vec[0] - 1;
    for &e in vec.iter() {
        if e != last + 1 {
            return last + 1;
        }
        last = e;
    }
    panic!("no answer");
}
