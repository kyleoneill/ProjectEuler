use std::time::{Instant};
const CEILING: u32 = 1000;

fn main() {
    let start = Instant::now();
    let solution = longest_reciprocal_cycle();
    println!("Found solution in {:?} microseconds", start.elapsed().as_micros());
    println!("The number with the longest reciprocal is {0} at length {1}", solution.0.to_string(), solution.1.to_string());
}

fn longest_reciprocal_cycle() -> (u32, u32) {
    let mut sequence_length = 0;
    let mut longest_reciprocal = 0;
    for x in (2 .. CEILING).rev() {
        if sequence_length >= x {
            break;
        }
        let mut found_remainders = vec![0; x as usize];
        let mut value: u32 = 1;
        let mut position = 0;
        while (found_remainders[value as usize] == 0) & (value != 0) {
            found_remainders[value as usize] = position;
            value *= 10;
            value %= x;
            position += 1;
        }
        if (position - found_remainders[value as usize]) > sequence_length {
            sequence_length = position - found_remainders[value as usize];
            longest_reciprocal = x;
        }
    }
    (longest_reciprocal, sequence_length)
}
