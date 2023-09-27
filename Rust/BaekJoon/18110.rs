// 18110번 solved.ac

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

fn read_i32() -> i32 {
    read_from_stdin()
        .parse()
        .expect("expected to input integer")
}

fn main() {
    let num_of_opinions = read_i32() as usize;
    let num_of_removed  = (num_of_opinions as f64 * 0.15).round() as usize;

    let mut opinions = Vec::with_capacity(num_of_opinions);

    (0..num_of_opinions).for_each(|_| {
        opinions.push(read_i32());
    });

    opinions.sort();

    let opinions = &opinions[num_of_removed .. opinions.len() - num_of_removed];
    let result   = (opinions.iter().sum::<i32>() as f64 / opinions.len() as f64).round() as usize;

    println!("{result}");
}