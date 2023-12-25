// 최소, 최대
use std::io;

fn main() {

    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read n");
    let n: usize = n.trim().parse().expect("Invalid n");

    let mut min = i32::MAX;
    let mut max = i32::MIN;
    
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read input");
    let numbers: Vec<i32> = input_line
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    for &num in &numbers {
        if num < min {
            min = num;
        }
        if num > max {
            max = num;
        }
    }

    println!("{} {}", min, max);
}
