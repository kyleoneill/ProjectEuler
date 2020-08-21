use std::time::{Instant};
const CEILING: i32 = 1000;

struct MaxConsecutive {
    consecutive_primes: u32,
    a: i32,
    b: i32
}

impl MaxConsecutive {
    fn new(consecutive_primes: u32, a: i32, b: i32) -> Self {
        MaxConsecutive {
            consecutive_primes,
            a,
            b
        }
    }
    fn new_consecutive(&mut self, consecutive_primes: u32, a: i32, b: i32) {
        self.consecutive_primes = consecutive_primes;
        self.a = a;
        self.b = b;
    }
    fn coefficient_product(&self) -> i32 {
        self.a * self.b
    }
}

fn main() {
    let start = Instant::now();
    let solution = quadratic_primes();
    println!("Found solution in {:?} microseconds", start.elapsed().as_micros());
    println!("The product of coefficients {} and {} is {} with a streak of {}", solution.a, solution.b, solution.coefficient_product(), solution.consecutive_primes);
}

fn quadratic_primes() -> MaxConsecutive {
    let mut max_consecutive_primes = MaxConsecutive::new(0, 0, 0);
    for a in (-CEILING + 1)..CEILING {
        for b in -CEILING..=CEILING {
            let mut current_consecutive_primes = 1;
            let mut n = 0;
            loop {
                let res = quadratric(n, a, b);
                if !is_prime(res) {
                    break;
                }
                n += 1;
                current_consecutive_primes += 1;
                if current_consecutive_primes > max_consecutive_primes.consecutive_primes {
                    max_consecutive_primes.new_consecutive(current_consecutive_primes, a, b);
                }
            }
        }
    }
    max_consecutive_primes
}

fn quadratric(n: i32, a: i32, b: i32) -> i32 {
    (n * n) + (a * n) + b
}

fn is_prime(n :i32) -> bool {
    if n <= 3 {
        return n > 1
    }
    else if (n % 2 == 0) || (n % 3 == 0) {
        return false
    }
    let mut i = 5;
    while i * i <= n {
        if (n % i == 0) || n % (i + 2) == 0 {
            return false
        }
        i = i + 6;
    }
    true
}
