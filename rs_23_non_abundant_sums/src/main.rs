use std::time::{Instant};
use std::collections::HashSet;

fn main() {
    let start = Instant::now();
    let abundant_numbers = get_abundant_numbers();
    let abundant_sums = sum_abundant_numbers(abundant_numbers);
    let solution = sum_non_abundants(abundant_sums);
    println!("Found solution in {:?} microseconds", start.elapsed().as_micros());
    println!("The solution is: {0}", solution.to_string());
}

fn sum_divisors(n: u32 ) -> u32 {
    let mut sum = 1;
    let upper_bound = (n as f32).sqrt().floor() as u32;
    for i in 2 ..= upper_bound {
        if n % i == 0 {
            sum += i;
            if i != n / i {
                sum += n / i;
            }
        }
    }
    sum
}

fn get_abundant_numbers() -> Vec<u32> {
    let mut abundant_numbers = Vec::new();
    for n in 12 ..= 28123 {  // test changing this to 28112 (28123 - 12)
        if sum_divisors(n) > n {
            abundant_numbers.push(n);
        }
    }
    abundant_numbers
}

fn sum_abundant_numbers(abundant_numbers: Vec<u32>) -> HashSet<u32> {
    let mut set = HashSet::new();
    for i in 0 .. abundant_numbers.len() { //get all unique pairs of the vector
        for j in i .. abundant_numbers.len() {
            let sum = abundant_numbers[i] + abundant_numbers[j];
            set.insert(sum);
        }
    }
    set
}

fn sum_non_abundants(abundant_sums: HashSet<u32>) -> u32 {
    let mut sum = 0;
    for n in 1 ..= 28123 {
        if !abundant_sums.contains(&n) {
            sum += n;
        }
    }
    sum
}
