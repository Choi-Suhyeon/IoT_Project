// 1158번 요세푸스 문제

use std::io;

fn read_from_stdin() -> String {
    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("failed to read");

    buffer
        .trim()
        .to_string()
}

fn read_usizes() -> Vec::<usize> {
    read_from_stdin()
        .split_whitespace()
        .map(|s| s
            .parse()
            .expect("expected to input integer")
        )
        .collect()
}

fn main() {
    let [n, k] = read_usizes()[..] else {
        panic!("Invalid input format");
    };
    let step   = k - 1;

    let mut init   = (1..=n).collect::<Vec<usize>>();
    let mut result = Vec::new();
    let mut idx    = step;

    while !init.is_empty() {
        result.push(init
            .remove(idx)
            .to_string()
        );

        idx = (idx + step)
            .checked_rem(init.len())
            .or(Some(0))
            .unwrap();
    }

    println!("<{}>", result.join(", "));
}
