use std::time::{Instant};
const PERMUTATION: u32 = 1_000_000;

fn main() {
    let start = Instant::now();
    let input = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let solution = generate_lex_permutation(input, PERMUTATION - 1);
    println!("Found solution in {:?} microseconds", start.elapsed().as_micros());
    println!("The solution is: {0}", solution.into_iter().collect::<String>());
}

fn generate_lex_permutation(mut input: Vec<char>, mut position: u32) -> Vec<char> {
    let mut answer = Vec::new();
    for n in (1 ..= input.len()).rev() {
        if input.len() == 1 {
            answer.push(input[0]);
            break;
        }
        let f = factorial((n - 1) as u32);
        let temp_position = (position / f) as usize;
        answer.push(input.remove(temp_position));
        position %= f;
    }
    answer
}

fn factorial(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => n * factorial(n - 1)
    }
}
