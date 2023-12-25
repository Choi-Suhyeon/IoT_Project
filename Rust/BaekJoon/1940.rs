// 1940번 주몽

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

fn read_usize() -> usize {
    read_from_stdin()
        .parse()
        .expect("expected to input integer")
}

fn read_usizes() -> Vec<usize> {
    read_from_stdin()
        .split_whitespace()
        .map(|x| x
            .parse()
            .expect("expected to input integers")
        )
        .collect()
}

fn main() {
    let _            = read_usize();
    let sum_expected = read_usize();

    let mut nums   = read_usizes();
    let mut result = 0;

    while !nums.is_empty() {
        if let Some(v) = nums.iter().rposition(|&x| x == sum_expected - nums[0]) {
            if v != 0 {
                result += 1;
                nums.remove(v);
            }
        }

        nums.remove(0);
    }

    println!("{result}");
}