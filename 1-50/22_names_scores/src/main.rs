use std::time::{Instant};
use std::fs::File;
use std::io::{prelude::*, BufReader};

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
];

fn main() {
    let start = Instant::now();
    let mut solution = 0;
    let mut input = read_input("input.txt");
    input.sort();
    for (pos, name) in input.iter().enumerate() {
        let alphabetic_value = get_alphabetic_value(name);
        solution += alphabetic_value * (pos + 1) as u64;
    }
    println!("Found solution in {:?} microseconds", start.elapsed().as_micros());
    println!("The solution is: {0}", solution.to_string());
}

fn get_alphabetic_value(name: &str) -> u64 {
    let mut unborrowed_name: String = name.clone().to_string();
    let mut sum: u64 = 0;
    unborrowed_name.make_ascii_lowercase();
    for c in unborrowed_name.chars() {
        let index = ASCII_LOWER.iter().position(|&r| r == c).unwrap() + 1;
        sum += index as u64;
    }
    sum
}

fn read_input(filename: &str) -> Vec<String> {
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    let mut file_contents: Vec<String> = Vec::new();
    for line in file.lines() {
        for name in line.unwrap().split(",") {
            file_contents.push(name.replace("\"", ""));
        }
    }
    file_contents
}
