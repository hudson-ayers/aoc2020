use std::fs::File;
use std::io::{self, BufRead};
use std::vec;
extern crate elapsed;
use elapsed::measure_time;

fn main() {
    let filename = "input/input.txt";
    let input = file_to_vec(filename);
    let (time, ret) = measure_time(|| solve_part_1(&input));
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

fn solve_part_1(numbers: &Vec<i64>) -> i64 {
    for (idx1, num1) in numbers.iter().enumerate() {
        for num2 in numbers[idx1 + 1..].iter() {
            if num1 + num2 == 2020 {
                return num1 * num2;
            }
        }
    }
    panic!("no solution part 1");
}

fn solve_part_2(numbers: &Vec<i64>) -> i64 {
    for (idx1, num1) in numbers.iter().enumerate() {
        for (idx2, num2) in numbers[idx1 + 1..].iter().enumerate() {
            for num3 in numbers[idx1 + idx2 + 2..].iter() {
                if num1 + num2 + num3 == 2020 {
                    return num1 * num2 * num3;
                }
            }
        }
    }
    panic!("no solution part 2");
}
