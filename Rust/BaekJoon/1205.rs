//1205번 등수 구하기

use std::process;
use std::io;

fn input_u32s() -> Vec<u32> {
    let mut raw_input = String::new();

    io::stdin()
        .read_line(&mut raw_input)
        .expect("failed to read");

    return raw_input
        .trim()
        .split(" ")
        .map(|s| s
            .parse::<u32>()
            .expect("expected to input int")
        )
        .collect()
}

fn main() {
    let [score_num, score, max_num] = input_u32s()[..]
    else {
        panic!("parse failed");
    };

    if score_num == 0 {
        print!("1");
        process::exit(0);
    }

    let mut scores = input_u32s();

    scores.push(score);
    scores.sort_by(|x, y| y.cmp(x));

    let nth_desc = 1 + scores
        .iter()
        .position(|r: &u32| *r == score)
        .unwrap();

    scores.sort();

    let score_idx = scores
        .iter()
        .position(|r: &u32| *r == score)
        .unwrap();
    let nth       = scores.len() - score_idx;

    print!("{}", if nth > max_num as usize { "-1".to_string() } else { nth_desc.to_string() });
}