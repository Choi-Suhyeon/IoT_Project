// 2745번 진법 변환

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

fn main() {
    let num_to_decimal = HashMap::<char, usize>::from([
        ('0', 0),  ('1', 1),  ('2', 2),  ('3', 3),  ('4', 4),  ('5', 5),
        ('6', 6),  ('7', 7),  ('8', 8),  ('9', 9),  ('A', 10), ('B', 11), 
        ('C', 12), ('D', 13), ('E', 14), ('F', 15), ('G', 16), ('H', 17), 
        ('I', 18), ('J', 19), ('K', 20), ('L', 21), ('M', 22), ('N', 23), 
        ('O', 24), ('P', 25), ('Q', 26), ('R', 27), ('S', 28), ('T', 29), 
        ('U', 30), ('V', 31), ('W', 32), ('X', 33), ('Y', 34), ('Z', 35),
    ]);
    
    let input           = read_from_stdin();
    let [number, radix] = input
        .split_whitespace()
        .collect::<Vec<_>>()[..]
    else {
        panic!("invalid input format");
    };

    let radix  = radix
        .parse::<usize>()
        .expect("invalid radix");
    let result = number
        .chars()
        .rev()
        .enumerate()
        .fold(0usize, |acc, (i, c)| acc + num_to_decimal[&c] * radix.pow(i as u32));

    println!("{result}");
}