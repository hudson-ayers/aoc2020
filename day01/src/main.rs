use std::fs::File;
use std::io::{self, BufRead};
use std::vec;

fn main() {
    let filename = "input/input.txt";
    let input = file_to_vec(filename);
    let ret = solve_part_1(&input);
    let ret2 = solve_part_2(&input);
    println!("part 1: {:?}", ret);
    println!("part 2: {:?}", ret2);
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
        for num2 in numbers.iter().skip(idx1 + 1) {
            if num1 + num2 == 2020 {
                return num1 * num2;
            }
        }
    }
    panic!("no solution");
}

fn solve_part_2(numbers: &Vec<i64>) -> i64 {
    for (idx1, num1) in numbers.iter().enumerate() {
        for (idx2, num2) in numbers.iter().skip(idx1 + 1).enumerate() {
            for num3 in numbers.iter().skip(idx1 + idx2 + 2) {
                if num1 + num2 + num3 == 2020 {
                    return num1 * num2 * num3;
                }
            }
        }
    }
    panic!("no solution");
}
