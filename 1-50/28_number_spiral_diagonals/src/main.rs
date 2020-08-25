#![allow(dead_code)]
use std::time::{Instant};
const SPIRAL_LENGTH: u32 = 1001;

struct Diagonal {
    pub square_length: u32,
    pub amount_to_add: u32
}

impl Diagonal {
    pub fn new() -> Self {
        Diagonal {
            square_length: 3,
            amount_to_add: 2
        }
    }
    pub fn increment(&mut self) {
        self.square_length += 2;
        self.amount_to_add += 2;
    }
}

fn main() {
    let start = Instant::now();
    let solution = fast_spiral_diagonals(SPIRAL_LENGTH);
    println!("Found solution in {:?} microseconds", start.elapsed().as_micros());
    println!("The answer is {}", solution);
}

//Second Attempt
//Found formula to add layers - Sum of 4(4n^2 + n + 1) where n is the diagonal length
fn fast_spiral_diagonals(spiral_length: u32) -> u32 {
    let len = (spiral_length + 1) / 2;
    let mut sum = 1;
    for n in 1..len {
        sum += (16 * square(n)) + (4 * n) + 4;
    }
    sum
}

//First Attempt
fn spiral_diagonals(spiral_length: u32) -> u32 {
    let mut diagonal = Diagonal::new();
    let mut running_sum = 1;
    let mut current_number = 3;
    loop {
        if diagonal.square_length > spiral_length {
            break;
        }
        running_sum += current_number;
        if square(diagonal.square_length) == current_number {
            diagonal.increment();
        }
        current_number += diagonal.amount_to_add;
    }
    running_sum
}

fn square(a: u32) -> u32 {
    a * a
}