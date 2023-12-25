// 2712번 미국 스타일n

use std::fmt::Write;
use std::io;

fn read_num_with_unit() -> (f64, String) {
    let mut raw_input = String::new();

    io::stdin()
        .read_line(&mut raw_input)
        .expect("failed to read");

    let result = raw_input
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>();

    (result[0].parse().unwrap(), String::from(result[1]))
}

fn read_usize() -> usize {
    let mut raw_input = String::new();

    io::stdin()
        .read_line(&mut raw_input)
        .expect("failed to read");

    raw_input
        .trim()
        .parse()
        .unwrap()
}

fn main() {
    let num_of_case = read_usize();

    let mut result = String::new();

    (0..num_of_case).for_each(|_| {
        let (num, unit) = read_num_with_unit();

        match &*unit {
            "kg" => writeln!(result, "{:.4} lb", 2.2046 * num),
            "lb" => writeln!(result, "{:.4} kg", 0.4536 * num),
            "l"  => writeln!(result, "{:.4} g", 0.2642 * num),
            "g"  => writeln!(result, "{:.4} l", 3.7854 * num),
            _    => panic!("unknown unit"),
        }
            .unwrap();
    });

    print!("{result}");
}