use std::time::{Instant};

const CEILING: u32 = 10000;

fn main() {
    let start = Instant::now();
    let mut solution = 0;
    for n in 2 .. CEILING {
        let tmp = sum_divisors(n);
        if n == sum_divisors(tmp) && n != tmp {
            solution += n;
        }
    }
    println!("Found solution in {:?} microseconds", start.elapsed().as_micros());
    println!("The solution is: {0}", solution.to_string());
}

fn sum_divisors(n: u32 ) -> u32 {
    let mut sum = 1;
    let upper_bound = (n as f32).sqrt().floor() as u32 + 1;
    for i in 2 ..= upper_bound {
        if n % i == 0 {
            sum += i;
            if i != upper_bound {
                sum += n / i;
            }
        }
    }
    sum
}
