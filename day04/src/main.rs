use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
extern crate elapsed;
use elapsed::measure_time;
use lazy_static::lazy_static;
use std::collections::HashMap;

// I feel like there is a cleaner solution (to part 2 in particular) that
// involves more skillful use of Result...also, the use of HashMap is probably
// less efficient for these inputs than if I just used a vector, but lookups are
// cleaner with HashMap

fn main() {
    let filename = "input/input.txt";
    let (time, ret) = measure_time(|| solve(&filename, passport_is_valid_part_1));
    println!("part 1: {} in {}", ret, time);
    let (time2, ret2) = measure_time(|| solve(&filename, passport_is_valid_part_2));
    println!("part 2: {} in {}", ret2, time2);
}

fn passport_is_valid_part_1(map: &HashMap<String, String>) -> bool {
    map.contains_key("byr")
        && map.contains_key("iyr")
        && map.contains_key("eyr")
        && map.contains_key("hgt")
        && map.contains_key("hcl")
        && map.contains_key("ecl")
        && map.contains_key("pid")
}

fn passport_is_valid_part_2(map: &HashMap<String, String>) -> bool {
    lazy_static! {
        static ref RE_INCHES: Regex = Regex::new(r"^([0-9]+)in$").unwrap();
        static ref RE_CM: Regex = Regex::new(r"^([0-9]+)cm$").unwrap();
        static ref RE_COLOR: Regex = Regex::new(r"^#[0-9|a-f]{6}$").unwrap();
        static ref RE_PID: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    }
    map.get("byr").map_or(false, |s| {
        s.parse::<usize>()
            .map_or(false, |yr| yr >= 1920 && yr <= 2002)
    }) && map.get("iyr").map_or(false, |s| {
        s.parse::<usize>()
            .map_or(false, |yr| yr >= 2010 && yr <= 2020)
    }) && map.get("eyr").map_or(false, |s| {
        s.parse::<usize>()
            .map_or(false, |yr| yr >= 2020 && yr <= 2030)
    }) && map.get("hgt").map_or(false, |s| {
        RE_INCHES.captures(s).map_or(false, |cap| {
            let hgt = cap.get(1).unwrap().as_str();
            let hgt = hgt.parse::<usize>().unwrap();
            hgt >= 59 && hgt <= 76
        }) || RE_CM.captures(s).map_or(false, |cap| {
            let hgt = cap.get(1).unwrap().as_str();
            let hgt = hgt.parse::<usize>().unwrap();
            hgt >= 150 && hgt <= 193
        })
    }) && map.get("hcl").map_or(false, |s| RE_COLOR.is_match(s))
        && map.get("ecl").map_or(false, |s| {
            s == "amb"
                || s == "blu"
                || s == "brn"
                || s == "gry"
                || s == "grn"
                || s == "hzl"
                || s == "oth"
        })
        && map.get("pid").map_or(false, |s| RE_PID.is_match(s))
}

fn solve<F: Fn(&HashMap<String, String>) -> bool>(filename: &str, validate: F) -> u64 {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut working_entry: HashMap<String, String> = HashMap::new();
    let mut valid_entries = 0;
    for line in lines {
        let string = line.unwrap();
        if string.is_empty() {
            if validate(&working_entry) {
                valid_entries += 1;
            }
            working_entry.clear();
        } else {
            working_entry.extend(string.split_ascii_whitespace().map(|s| {
                let (k, v) = s.split_at(s.find(":").unwrap());
                (k.to_string(), v[1..].to_string())
            }));
        }
    }
    // file does not have blank line at end
    if validate(&working_entry) {
        valid_entries += 1;
    }
    valid_entries
}
