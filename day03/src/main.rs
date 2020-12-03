use std::fs::File;
use std::io::{self, BufRead};
extern crate elapsed;
use elapsed::measure_time;

fn main() {
    let filename = "input/input.txt";
    let (time, ret) = measure_time(|| part_1(&filename, 3, 1));
    println!("part 1: {} in {}", ret, time);
    let (time2, ret2) = measure_time(|| part_2(&filename));
    println!("part 2: {} in {}", ret2, time2);
}

// Approach Ideas:
// 1. Read the whole thing into memory in an 2d array of bools (or 2d vec or w.e.)
//    Then iterate through the array and maintain my current position. Treat passing too far right
//    as wrapping back to position 0. ez-pz
//    2. Buffered read, update position while reading, count for each line if i hit a tree.

struct Acc {
    trees: u64,
    x_pos: usize,
    y_pos: usize,
}

fn part_1(filename: &str, x_slope: usize, y_slope: usize) -> u64 {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut line_width = 0; // to support addl widths just read width of first line
    lines
        .fold(
            Acc {
                trees: 0,
                x_pos: 0,
                y_pos: 0,
            },
            |mut acc, line| {
                if acc.y_pos % y_slope != 0 {
                    acc.y_pos += 1;
                    acc
                } else {
                    let line = line.unwrap();
                    line_width = line.len();

                    if line.bytes().nth(acc.x_pos as usize) == Some(b'#') {
                        acc.trees += 1;
                    }
                    acc.x_pos += x_slope;
                    if acc.x_pos >= line_width {
                        acc.x_pos = acc.x_pos - line_width;
                    }
                    acc.y_pos += 1;
                    acc
                }
            },
        )
        .trees
}

fn part_2(filename: &str) -> u64 {
    part_1(filename, 1, 1)
        * part_1(filename, 3, 1)
        * part_1(filename, 5, 1)
        * part_1(filename, 7, 1)
        * part_1(filename, 1, 2)
}
