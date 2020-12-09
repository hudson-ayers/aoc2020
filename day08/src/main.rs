use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;
extern crate elapsed;
use elapsed::measure_time;

fn main() {
    let filename = "input/input.txt";
    let (time, ret) = measure_time(|| part_1(&filename));
    println!("part 1: {} in {}", ret, time);
    let (time2, ret2) = measure_time(|| part_2(&filename));
    println!("part 2: {} in {}", ret2, time2);
}

fn parse(filename: &str) -> Vec<(String, isize)> {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines
        .map(|line| {
            let string = line.unwrap();
            let mut split = string.split_ascii_whitespace();
            let op = split.next().unwrap();
            let arg = split.next().unwrap().parse::<isize>().unwrap();
            (op.to_string(), arg)
        })
        .collect()
}

/// Ok() --> Found repeat instruction
/// Err() --> reached end of instructions
fn part_1_logic(instrs: &Vec<(String, isize)>) -> Result<isize, isize> {
    let mut acc: isize = 0;
    let mut next: isize = 0;
    let mut found: Vec<bool> = vec![false; instrs.len()];
    loop {
        if next as usize == found.len() {
            break;
        }
        if found[next as usize] {
            return Ok(acc);
        }
        found[next as usize] = true;
        let (op, arg) = &instrs[next as usize];
        match op.as_str() {
            "acc" => {
                acc += arg;
                next += 1;
            }
            "jmp" => {
                next = next + arg;
            }
            "nop" => {
                next += 1;
            }
            _ => panic!("invalid op"),
        }
    }
    Err(acc)
}

fn part_1(filename: &str) -> isize {
    let instrs = parse(filename);
    part_1_logic(&instrs).unwrap()
}

// Very brute force solution :/
fn part_2(filename: &str) -> isize {
    let instrs = parse(filename);
    for (i, (op, _)) in instrs.iter().enumerate() {
        match op.as_str() {
            "acc" => {}
            "jmp" => {
                let mut clone = instrs.clone();
                clone[i].0 = "nop".to_string();
                match part_1_logic(&clone) {
                    Ok(_) => continue,
                    Err(acc) => {
                        return acc;
                    }
                }
            }
            "nop" => {
                let mut clone = instrs.clone();
                clone[i].0 = "jmp".to_string();
                match part_1_logic(&clone) {
                    Ok(_) => continue,
                    Err(acc) => {
                        return acc;
                    }
                }
            }
            _ => panic!("invalid op"),
        }
    }
    panic!("no solution");
}
