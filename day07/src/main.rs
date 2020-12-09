use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;
extern crate elapsed;
use elapsed::measure_time;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    let filename = "input/input.txt";
    let (time, ret) = measure_time(|| part_1(&filename));
    println!("part 1: {} in {}", ret, time);
    let (time2, ret2) = measure_time(|| part_2(&filename));
    println!("part 2: {} in {}", ret2, time2);
}

fn part_1(filename: &str) -> usize {
    let bag_map = parse(filename);
    let mut outer_set: HashSet<String> = HashSet::new();
    for (k, v) in bag_map.iter() {
        for (name, _) in v.iter() {
            if name == "shiny gold" {
                outer_set.insert(k.clone());
            }
        }
    }
    loop {
        let cur_size = outer_set.len();
        for item in outer_set.clone().iter() {
            for (k, v) in bag_map.iter() {
                for (name, _) in v.iter() {
                    if name == item {
                        outer_set.insert(k.clone());
                    }
                }
            }
        }
        if outer_set.len() == cur_size {
            break;
        }
    }
    outer_set.len()
}

fn count_bags_in(name: &String, bag_map: &HashMap<String, Vec<(String, usize)>>) -> usize {
    let v = bag_map.get(name).unwrap();
    if v.is_empty() {
        0
    } else {
        v.iter().fold(0, |acc, (name, num)| {
            acc + (1 + count_bags_in(&name, bag_map)) * num
        })
    }
}

fn part_2(filename: &str) -> usize {
    let bag_map = parse(filename);
    count_bags_in(&"shiny gold".to_string(), &bag_map)
}

fn parse(filename: &str) -> HashMap<String, Vec<(String, usize)>> {
    lazy_static! {
        static ref INIT_RE: Regex =
            Regex::new(r"^([a-z ]+) bags contain (.*)([0-9]|no) ([a-z ]+) bags?.$").unwrap();
        static ref INNER_RE: Regex = Regex::new(r"([0-9]) ([a-z ]+) bags?, ").unwrap();
    }
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut bag_map: HashMap<String, Vec<(String, usize)>> = HashMap::new();

    for line in lines {
        let string = line.unwrap();
        let cap = INIT_RE.captures(&string).unwrap();
        let outer_bag_color = cap.get(1).unwrap().as_str();
        let inner_bag_list = cap.get(2).unwrap().as_str();
        let mut vec = Vec::new();
        for caps in INNER_RE.captures_iter(inner_bag_list) {
            vec.push((caps[2].to_string(), caps[1].parse::<usize>().unwrap()));
        }
        let final_bag_num = cap.get(3).unwrap().as_str(); //can be none if "no other"
        let final_bag_color = cap.get(4).unwrap().as_str();
        if !(final_bag_num == "no") {
            vec.push((
                final_bag_color.to_string(),
                final_bag_num.parse::<usize>().unwrap(),
            ));
        }
        bag_map.insert(outer_bag_color.to_string(), vec);
    }
    bag_map
}
