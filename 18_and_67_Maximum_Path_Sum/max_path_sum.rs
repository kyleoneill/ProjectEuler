use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::cmp;
use std::time::{Instant};

struct Triangle {
    data: Vec<i32>,
    rows: u32
}

impl Triangle {
    fn get_cell_position(row: u32, cell: u32)  -> usize {
        let triangle_number = row * (row + 1) / 2;
        (triangle_number + cell) as usize
    }
    fn get_value_at_position(&self, row: u32, cell: u32) -> i32 {
        let position = Triangle::get_cell_position(row, cell);
        self.data[position]
    }
    fn set_value_at_position(&mut self, row: u32, cell: u32, new_value: i32) {
        let position = Triangle::get_cell_position(row, cell);
        self.data[position] = new_value;
    }
}

fn main() {
    let start = Instant::now();
    let triangle: Triangle = read_input("input.txt");
    let solution = process_triangle(triangle);
    println!("Found solution in {:?} microseconds", start.elapsed().as_micros());
    println!("The solution is: {0}", solution.to_string());
}

fn read_input(filename: &str) -> Triangle {
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    let mut triangle: Vec<i32> = Vec::new();
    let mut num_of_rows = 0;
    for line in file.lines() {
        num_of_rows += 1;
        for num in line.unwrap().split(" ") {
            let num = num.parse::<i32>().unwrap();
            triangle.push(num);
        }
    }
    Triangle {data: triangle, rows: num_of_rows}
}

fn process_triangle(mut triangle: Triangle) -> i32 {
    for current_row in (0..=triangle.rows - 2).rev() {
        for current_position in 0..=current_row {
            let max_num = cmp::max(triangle.get_value_at_position(current_row + 1, current_position), triangle.get_value_at_position(current_row + 1, current_position + 1));
            let new_value = triangle.get_value_at_position(current_row, current_position) + max_num;
            triangle.set_value_at_position(current_row, current_position, new_value);
        }
    }
    triangle.get_value_at_position(0, 0)
}
