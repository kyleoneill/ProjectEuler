use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::time::{Instant};

fn main() {
    let date_range = read_input("input.txt");
    println!("Finding solution between years {0} and {1}.", date_range.0.to_string(), date_range.1.to_string());
    let start = Instant::now();
    let solution = count_sundays(date_range);
    println!("Found solution in {:?} microseconds", start.elapsed().as_micros());
    println!("The solution is: {0}", solution.to_string());
}

fn read_input(filename: &str) -> (i32, i32) {
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    let mut file_contents: Vec<String> = Vec::new();
    for line in file.lines() {
        for word in line.unwrap().split(" ") {
            file_contents.push(word.to_string());
        }
    }
    (file_contents[2].parse::<i32>().unwrap(), file_contents[6].parse::<i32>().unwrap())
}

fn count_sundays(date_range: (i32, i32)) -> i32 {
    let mut num_of_sundays = 0;
    for year in date_range.0 ..= date_range.1 {
        for month in 0 .. 12 {
            if is_weekday_of_first_of_month_sunday(month, year) {
                num_of_sundays += 1;
            }
        }
    }
    num_of_sundays
}

fn is_weekday_of_first_of_month_sunday(month: i32, mut year: i32) -> bool {
    if month == 0 || month == 1 {
        year -= 1;
    }
    let month = greg_to_zeller(month);
    let d = year % 100;
    let c = year / 100;
    let f = 1 + ((13 * month - 1) / 5) + d + (d / 4) + (c / 4) - (2 * c);
    let mut remainder = f % 7;
    if remainder < 0 {
        remainder += 7;
    }
    if remainder == 0 {
        true
    }
    else {
        false
    }
}

fn greg_to_zeller(month: i32) -> i32 {
    match month {
        0 => 11,
        1 => 12,
        2 => 1,
        3 => 2,
        4 => 3,
        5 => 4,
        6 => 5,
        7 => 6,
        8 => 7,
        9 => 8,
        10 => 9,
        11 => 10,
        _ => panic!("Invalid input")
    }
}
