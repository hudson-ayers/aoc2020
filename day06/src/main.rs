use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;
extern crate elapsed;
use elapsed::measure_time;
use std::collections::{HashMap, HashSet};

// Note: array based solution, with each slot in array representing a char,
// would be more efficient but less idiomatic and harder to write.

fn main() {
    let filename = "input/input.txt";
    let (time, ret) = measure_time(|| solve(&filename));
    println!("part 1: {} in {}", ret, time);
    let (time2, ret2) = measure_time(|| part_2(&filename));
    println!("part 2: {} in {}", ret2, time2);
}

fn solve(filename: &str) -> usize {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut working_entry: HashSet<char> = HashSet::new();
    let mut sums: Vec<usize> = vec![];

    for line in lines {
        let string = line.unwrap();
        if string.is_empty() {
            sums.push(working_entry.iter().count());
            working_entry.clear();
        } else {
            for c in string.chars() {
                working_entry.insert(c);
            }
        }
    }
    // final line of input is not blank
    sums.push(working_entry.iter().count());
    sums.iter().sum()
}

fn part_2(filename: &str) -> usize {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut working_entry: HashMap<char, usize> = HashMap::new();
    let mut num_entries: usize = 0;
    let mut sums: Vec<usize> = vec![];

    for line in lines {
        let string = line.unwrap();
        if string.is_empty() {
            sums.push(
                working_entry
                    .values()
                    .filter(|&&v| v == num_entries)
                    .count(),
            );
            working_entry.clear();
            num_entries = 0;
        } else {
            num_entries += 1;
            for c in string.chars() {
                match working_entry.get(&c) {
                    Some(&val) => {
                        working_entry.insert(c, val + 1);
                    }
                    None => {
                        working_entry.insert(c, 1);
                    }
                }
            }
        }
    }
    // final line of input is not blank
    sums.push(
        working_entry
            .values()
            .filter(|&&v| v == num_entries)
            .count(),
    );
    sums.iter().sum()
}
