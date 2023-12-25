// 15829번 Hashing

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

fn get_each_term(base: u128, exp: u128, modulo: u128, elem: u128) -> u128 {
    if exp == 0 { return elem; }

    let interim = (0..exp - 1)
        .fold(base, |acc, _| acc * base % modulo);

    interim * elem % modulo
}

fn main() {
    read_from_stdin();

    let target_str     = read_from_stdin();
    let al_num_hashmap = ('a'..='z')
        .enumerate()
        .map(|(n, v)| (v, n as u128 + 1))
        .collect::<HashMap<char, u128>>();
    let hash_value     = target_str
        .chars()
        .enumerate()
        .map(|(i, c)| get_each_term(31, i as u128, 1234567891, *al_num_hashmap.get(&c).unwrap()))
        .fold(0u128, |acc, x| (acc + x) % 1234567891);

    println!("{hash_value}");
}