// 10773번 제로

use std::io;

fn read_from_stdin() -> String {
    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("failed to read");

    buffer
        .trim()
        .to_string()
}

fn read_u32() -> u32 {
    read_from_stdin()
        .parse()
        .expect("expected to input integer")
}

fn main() {
    let num_of_nums = read_u32();

    let mut nums = Vec::with_capacity(num_of_nums as usize);

    for _ in 0..num_of_nums {
        let num = read_u32();

        if num != 0 { nums.push(num); }
        else        { nums.pop(); }
    }

    println!("{}", nums.iter().sum::<u32>());
}

/*
// 시간도 훨씬 덜 걸렸고 코드도 많이 짧아 옮겨놓음.
// 다른 풀이 1
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut stack = vec![];
    for next in buf.split_ascii_whitespace().skip(1).flat_map(str::parse::<u32>) {
        match next {
            0 => {stack.pop();},
            value => stack.push(value)
        }
    }
    println!("{}", stack.iter().sum::<u32>());
}


// 다른 풀이 2
use std::io::{self, BufWriter, Read, Write};

fn main() {
    let buf = {
        let mut tmp = String::new();
        io::stdin().read_to_string(&mut tmp).unwrap();
        tmp
    };
    let mut out = BufWriter::new(std::io::stdout());
    let mut iter = buf.lines().map(|s| s.parse::<u32>().unwrap());
    iter.next();

    let mut stack = Vec::new();
    for i in iter {
        if i == 0 {
            stack.pop();
        } else {
            stack.push(i);
        }
    }

    writeln!(out, "{}", stack.into_iter().sum::<u32>()).unwrap();
}
*/