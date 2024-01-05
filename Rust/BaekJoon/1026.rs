// 1026번 보물

use std::cmp::Reverse;
use std::fmt::Debug;
use std::str;
use std::io;

fn read_one<T>() -> T
where
    T: str::FromStr,
    T::Err: Debug,
{
    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("failed to read from stdin");

    buffer
        .trim()
        .parse()
        .expect("invalid input format")
}

fn read_some<T>() -> Vec<T>
where
    T: str::FromStr,
    T::Err: Debug,
{
    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("failed to read from stdin");

    buffer
        .trim()
        .split_whitespace()
        .map(|s| s
            .parse()
            .expect("invalid input format")
        )
        .collect()
}

fn main() {
    read_one::<usize>();

    let mut array_a = read_some::<usize>();
    let mut array_b = read_some::<usize>();

    array_a.sort();
    array_b.sort_by_key(|&u| Reverse(u));

    let result = array_a
        .iter()
        .zip(array_b)
        .map(|(x, y)| x * y)
        .sum::<usize>();

    println!("{}", result);
}
