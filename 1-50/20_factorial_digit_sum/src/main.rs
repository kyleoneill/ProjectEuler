use std::time::{Instant};
extern crate num_bigint;
extern crate num_traits;
use num_bigint::BigUint;
use num_traits::{Zero, One};

fn main() {
    let one_hundred: BigUint = 100u32.into();
    let start = Instant::now();
    let factorial = factorial(&one_hundred);
    let solution = sum_factorial_digits(factorial);
    println!("Found solution in {:?} microseconds", start.elapsed().as_micros());
    println!("The solution is: {0}", solution.to_string());
}

fn factorial(n: &BigUint) -> BigUint {
    if n == &Zero::zero() {
        One::one()
    }
    else if n == &One::one() {
        One::one()
    }
    else {
        let one: BigUint = One::one();
        factorial(&(n - one)) * n
    }
}

fn sum_factorial_digits(mut f: BigUint ) -> BigUint {
    let mut sum: BigUint = Zero::zero();
    while f != Zero::zero() {
        let ten: BigUint = 10u32.into();
        sum += &f % &ten;
        f /= ten;
    }
    sum
}
