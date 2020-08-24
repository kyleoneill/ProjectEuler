use std::time::Instant;
use std::collections::{HashSet, HashMap};
const MIN: u32 = 2;
const MAX: u32 = 100;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
struct ExponentNumber {
    base: u32,
    exponent: u32
}

impl ExponentNumber {
    fn new(base: u32, exponent: u32) -> Self {
        ExponentNumber {
            base,
            exponent
        }
    }
}

fn main() {
    let start = Instant::now();
    let solution = distinct_powers(MIN, MAX);
    println!("Found solution in {:?} microseconds", start.elapsed().as_micros());
    println!("The answer is {}", solution);
}

fn distinct_powers(min: u32, max: u32) -> u32 {
    let mut nums: HashSet<Vec<ExponentNumber>> = HashSet::new();
    for i in min..=max {
        for j in min..=max {
            nums.insert(prime_factorization(i, j));
        }
    }
    nums.len() as u32
}

fn prime_factorization(num: u32, exponent: u32) -> Vec<ExponentNumber> {
    let mut prime_factorizations: Vec<ExponentNumber> = Vec::new();
    let factorized_base = factorize_base(num);
    for (key, value) in factorized_base {
        let exponent_number = ExponentNumber::new(key, value * exponent);
        prime_factorizations.push(exponent_number);
    }
    prime_factorizations.sort();
    prime_factorizations
}

fn factorize_base(mut base: u32) -> HashMap<u32, u32> {
    let mut factors: HashMap<u32, u32> = HashMap::new();
    while base % 2 == 0 {
        let count = factors.entry(2).or_insert(0);
        *count += 1;
        base /= 2;
    }
    let mut i = 3;
    while square(i) <= base {
        while base % i == 0 {
            let count = factors.entry(i).or_insert(0);
            *count += 1;
            base /= i;
        }
        i += 2;
    }
    if base > 2 {
        let count = factors.entry(base).or_insert(0);
        *count += 1;
    }
    factors
}

fn square(n: u32) -> u32 {
    n * n
}
