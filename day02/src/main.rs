use std::fs::File;
use std::io::{self, BufRead};
extern crate elapsed;
use elapsed::measure_time;

fn main() {
    let filename = "input/input.txt";
    let (time, ret) = measure_time(|| part_1(&filename));
    println!("part 1: {} in {}", ret, time);
    let (time2, ret2) = measure_time(|| part_2(&filename));
    println!("part 2: {} in {}", ret2, time2);
}

fn part_1(filename: &str) -> i64 {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut correct = 0;
    for line in lines {
        let a = line.unwrap();
        let mut tokens = a.split_ascii_whitespace();
        let mut first = tokens.next().unwrap().split('-');
        let min: usize = first.next().unwrap().parse().unwrap();
        let max: usize = first.next().unwrap().parse().unwrap();
        let required = tokens.next().unwrap().chars().next().unwrap();
        let password = tokens.next().unwrap();
        let count = password.matches(required).count();
        if count >= min && count <= max {
            correct += 1;
        }
    }
    correct
}

fn part_2(filename: &str) -> i64 {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut correct = 0;
    for line in lines {
        let a = line.unwrap();
        let mut tokens = a.split_ascii_whitespace();
        let mut first = tokens.next().unwrap().split('-');
        let pos1: usize = first.next().unwrap().parse().unwrap();
        let pos2: usize = first.next().unwrap().parse().unwrap();
        let required = tokens.next().unwrap().chars().next().unwrap();
        let password = tokens.next().unwrap();

        let mut chars = password.chars();
        let match1 = match chars.nth(pos1 - 1) {
            Some(c) => c == required,
            None => false,
        };
        let match2 = match chars.nth(pos2 - pos1 - 1) {
            Some(c) => c == required,
            None => false,
        };

        if match1 ^ match2 {
            correct += 1;
        }
    }
    correct
}
