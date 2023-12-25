// 1212번 8진수 2진수

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

fn rm_prefix_zero(s: String) -> String {
    match s.strip_prefix('0') {
        Some(v) => rm_prefix_zero(v.to_string()),
        None    => s,
    }
}

fn main() {
    let oct_to_bin = HashMap::from([
        ('0', "000"), ('1', "001"), ('2', "010"), ('3', "011"),
        ('4', "100"), ('5', "101"), ('6', "110"), ('7', "111"),
    ]);
    let octal      = read_from_stdin();
    let interim    = octal
        .chars()
        .map(|x| oct_to_bin
            .get(&x)
            .unwrap()
            .to_string()
        )
        .collect::<Vec<_>>()
        .join("");
    let result     =
        if interim == "000" { String::from("0") }
        else                { rm_prefix_zero(interim) };

    println!("{result}");
}