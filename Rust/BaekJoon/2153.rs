// 2153번 소수 단어

use std::collections::HashMap;
use std::io;

fn read_from_stdin() -> String {
    let mut raw_input = String::new();

    io::stdin()
        .read_line(&mut raw_input)
        .expect("failed to read");

    raw_input
        .trim()
        .to_string()
}

fn is_prime_number(num: u64) -> bool {
    for i in 2..=(num as f64).sqrt() as u64 {
        if num % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    let input_string = read_from_stdin();
    let mapping      = ('a'..='z')
        .chain('A'..='Z')
        .zip(1..=52u64)
        .collect::<HashMap<_, _>>();
    let sum_of_chars = input_string
        .chars()
        .fold(0u64, |acc, x|
            mapping
                .get(&x)
                .or(Some(&0))
                .unwrap()
            + acc
        );

    println!("It is{}a prime word.", if is_prime_number(sum_of_chars) { " " } else { " not " });
}