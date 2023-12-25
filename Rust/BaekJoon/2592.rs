// 2592번 대표값

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

fn read_usize() -> usize {
    read_from_stdin()
        .parse()
        .expect("expected to input integer")
}

fn main() {
    let mut nums = [0usize; 10];

    for i in 0..nums.len() {
        nums[i] = read_usize();
    }

    let mean = nums.iter().sum::<usize>() / nums.len();
    let mode = **(nums
        .iter()
        .fold(HashMap::with_capacity(nums.len()), |mut map, n| {
            map
                .entry(n)
                .and_modify(|frq| *frq += 1)
                .or_insert(1);

            map
        })
        .iter()
        .max_by(|&(&num1, cnt1), &(&num2, cnt2)| {
            cnt1.cmp(cnt2).then(num1.cmp(num2))
        })
        .unwrap()
        .0
    );

    println!("{mean}\n{mode}");
}