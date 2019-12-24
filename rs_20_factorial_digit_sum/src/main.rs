use std::time::{Instant};

fn main() {
    let start = Instant::now();
    let factorial = factorial(100);
    let solution = sum_factorial_digits(factorial);
    println!("Found solution in {:?} microseconds", start.elapsed().as_micros());
    println!("The solution is: {0}", solution.to_string());
}

fn factorial(n: u128) -> u128 { // need to use a bigint here
    match n {
        0 => 1,
        1 => 1,
        _ => factorial(n - 1) * n,
    }
}

fn sum_factorial_digits(mut f: u128 ) -> u128 {
    let mut sum = 0;
    while f != 0 {
        sum += f % 10;
        f /= 10;
    }
    sum
}
