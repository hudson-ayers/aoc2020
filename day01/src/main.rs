use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec;
extern crate elapsed;
use elapsed::measure_time;

fn main() {
    let filename = "input/input.txt";
    let input = file_to_vec(filename);
    let (time, ret) = measure_time(|| solve_part_1(&input, 2020).unwrap());
    let (time2, ret2) = measure_time(|| solve_part_2(&input));
    println!("part 1: {} in {}", ret, time);
    println!("part 2: {} in {}", ret2, time2);
}

fn file_to_vec(filename: &str) -> Vec<i64> {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut numbers = vec![];
    for line in lines {
        numbers.push(line.unwrap().trim().parse::<i64>().unwrap());
    }
    numbers
}

// (O(n))
fn solve_part_1(numbers: &[i64], target: i64) -> Option<i64> {
    let mut set = HashSet::new();
    set.insert(numbers[0]);
    for num in numbers[0..].iter() {
        if set.contains(&(target - num)) {
            return Some(num * (target - num));
        } else {
            set.insert(*num);
        }
    }
    None
}

// O(n^2)
fn solve_part_2(numbers: &[i64]) -> i64 {
    for (i, num) in numbers[0..].iter().enumerate() {
        match solve_part_1(&numbers[i..], 2020 - num) {
            Some(res) => return res * num,
            None => {}
        }
    }
    panic!("no solution part 2");
}
