use std::time::Instant;
const POWER: u32 = 5;
fn main() {
    let start = Instant::now();
    let solution = sum_digit_powers();
    println!("Found solution in {} microseconds", start.elapsed().as_micros());
    println!("The solution is {}", solution);
}

fn sum_digit_powers() -> u64 {
    let digit_powers = get_digit_powers(POWER);
    let mut sum = 0;
    for n in digit_powers {
        sum += n;
    }
    sum
}

fn get_digit_powers(p: u32) -> Vec<u64> {
    let max_digits = max_digits_for_power(p);
    let max_digit_sum = max_possible_digit_sum(max_digits as u64, p);
    let mut matching_nums: Vec<u64> = Vec::new();
    for n in 2..max_digit_sum {
        let sum = sum_powers_of_digits(n, p);
        if sum == n {
            matching_nums.push(sum);
        }
    }
    matching_nums
}

fn sum_powers_of_digits(n: u64, p: u32) -> u64 {
    let mut sum: u64 = 0;
    let mut n = n;
    while n != 0 {
        sum += (n % 10).pow(p);
        n /= 10;
    }
    sum
}

fn max_digits_for_power(p: u32) -> u32 {
    let mut digit = 1;
    loop {
        let sum = max_possible_digit_sum(digit, p);
        if digit > get_num_of_digits(sum) {
            return (digit - 1) as u32
        }
        digit += 1;
        if digit > 99 {
            panic!("Could not find max digits for power");
        }
    }
}

fn max_possible_digit_sum(digit: u64, p: u32) -> u64 {
    digit * (9 as u64).pow(p)
}

fn get_num_of_digits(n: u64) -> u64 {
    let mut n = n;
    let mut count = 0;
    while n != 0 {
        n /= 10;
        count += 1;
    }
    count
}
