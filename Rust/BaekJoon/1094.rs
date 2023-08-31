//1094번 막대기

use std::cmp::Ordering;
use std::io;

fn main() {
    let mut raw_input = String::new();

    io::stdin()
        .read_line(&mut raw_input)
        .expect("failed to read");

    let     input  = raw_input
        .trim()
        .parse::<u8>()
        .expect("expected to input int");
    let mut sticks = vec![64u8];

    loop {
        match sticks.iter().sum::<u8>().cmp(&input) {
            Ordering::Greater => {
                sticks.sort();

                let min_half = sticks[0] >> 1;
                let _        = std::mem::replace(&mut sticks[0], min_half);

                if sticks.iter().sum::<u8>() < input {
                    sticks.push(min_half);
                }
            },
            Ordering::Less => panic!("do not solve"),
            Ordering::Equal => break,
        }
    }

    print!("{}", sticks.len());
}