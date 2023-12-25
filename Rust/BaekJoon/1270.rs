// 1270번 전쟁 - 땅따먹기

use std::collections::HashMap;
use std::fmt::Write;
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

fn read_i128s() -> Vec<i128> {
    read_from_stdin()
        .split_whitespace()
        .map(|x| x
            .parse()
            .expect("expected to input integers")
        )
        .collect()
}

fn main() {
    let num_of_cases = read_usize();

    let mut result = String::new();

    for _ in 0..num_of_cases {
        let raw_input   = read_i128s();
        let (&length, nums) = raw_input
            .split_first()
            .unwrap();
        let half_length = length as f64 / 2.;
        let frequencies = nums
            .iter()
            .fold(HashMap::with_capacity(length as usize), |mut map, x| {
                map
                    .entry(*x)
                    .and_modify(|freq| *freq += 1)
                    .or_insert(1u64);

                map
            });
        let majorities  = frequencies
            .iter()
            .filter(|&(_, &cnt)| cnt as f64 > half_length)
            .collect::<Vec<_>>();
        let interim     =
            if majorities.len() == 1 { majorities[0].0.to_string() }
            else                     { String::from("SYJKGW") };

        writeln!(result, "{}", interim).unwrap();
    }

    print!("{result}");
}