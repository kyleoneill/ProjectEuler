use std::time::{Instant};
extern crate num_bigint;
extern crate num_traits;
use num_bigint::BigUint;
use num_traits::{Zero, One};
use std::mem;

struct Fibonacci {
    curr: BigUint,
    next: BigUint,
}

impl Iterator for Fibonacci {
    type Item = BigUint;
    fn next(&mut self) -> Option<BigUint> {
        mem::swap(&mut self.curr, &mut self.next);
        self.next += self.curr.clone();
        Some(self.curr.clone())
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: Zero::zero(), next: One::one() }
}

fn main() {
    let start = Instant::now();
    let solution = get_fib_until_digit_amount(1000 as usize);
    println!("Found solution in {:?} microseconds", start.elapsed().as_micros());
    println!("The solution is: {0}", solution.to_string());
}

fn get_fib_until_digit_amount(n: usize) -> u128 {
    let mut fib_iter = fibonacci();
    let mut index = 1;
    loop {
        let current_fib: BigUint = fib_iter.next().unwrap();
        let length = current_fib.to_str_radix(10).chars().count();
        if length >= n {
            break
        }
        index += 1;
    }
    index
}
