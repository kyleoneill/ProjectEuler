use std::time::Instant;

//See - dynamic programming (https://www.geeksforgeeks.org/dynamic-programming/)

fn main() {
    let start = Instant::now();
    let solution = coin_sums(200);
    println!("Found solution in {} microseconds", start.elapsed().as_micros());
    println!("The solution is {}", solution);
}

fn coin_sums(n: i32) -> i32 {
    let coins = vec![1, 2, 5, 10, 20, 50, 100, 200];
    let m = coins.len();
    solve_fast(&coins, m, n)
}


//bottom-up dynamic solution
#[allow(dead_code)]
fn solve_fast(coins: &Vec<i32>, m: usize, n: i32) -> i32 {
    //table with n + 1 rows and m columns
    let mut table = vec![vec![0; m]; (n + 1) as usize];

    //fill entries for 0
    for i in 0..m {
        table[0][i] = 1;
    }

    //fill the rest of the entries from the bottom up
    for i in 1..=n {
        for j in 0..m {
            //count of solutions including coins[j]
            let x = if i - coins[j] >= 0 {table[(i - coins[j]) as usize][j]} else {0};
            
            //count of solutions excluding coins[j]
            let y = if j >= 1 {table[i as usize][(j - 1) as usize]} else {0};

            //total count
            table[i as usize][j] = x + y;
        }
    }
    return table[n as usize][m - 1 as usize] as i32
}

//recursive solution
#[allow(dead_code)]
fn solve_slow(coins: &Vec<i32>, m: usize, n: i32) -> i32 {
    if n == 0 {
        //If n is 0 there is one solution
        return 1
    }
    if n < 0 {
        //If n is less than 0 than no solution exists
        return 0
    }
    if m <= 0 && n >= 1 {
        //If there are no coins and n is greater than 0 then no solution exists
        return 0
    }

    //solve_slow(&coins, m - 1, n) = call solve_slow and ignore the last coin in coins
    //solve_slow(&coins, m, n - coins[m - 1]) = call solve_slow and reduce n by the value of the last value in coins
    return solve_slow(&coins, m - 1, n) + solve_slow(&coins, m, n - coins[m - 1])
}