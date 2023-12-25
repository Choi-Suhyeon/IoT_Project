// 2703번 Cryptoquote

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

fn main() {
    let num_of_cases = read_usize();

    let mut result = String::new();

    for _ in 0..num_of_cases {
        let cypher_text = read_from_stdin();
        let table       = read_from_stdin().into_bytes();
        let plain_text  = cypher_text
            .chars()
            .map(|c|
                if ('A'..='Z').contains(&c) { table[c as usize - 'A' as usize] as char }
                else                        { c }
            )
            .collect::<String>();

        writeln!(&mut result, "{}", plain_text).unwrap();
    }

    println!("{result}");
}