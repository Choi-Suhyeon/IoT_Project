// 10845번 큐

use std::collections::VecDeque;
use std::io;

fn main() {
    let mut raw_input = String::new();
    let mut queue     = VecDeque::new();

    io::stdin()
        .read_line(&mut raw_input)
        .expect("failed to read");

    let instructions_num = raw_input
        .trim()
        .parse::<i32>()
        .expect("failed to parse");

    for _ in 0..instructions_num {
        raw_input.clear();

        io::stdin()
            .read_line(&mut raw_input)
            .expect("failed to read");

        let instruction = raw_input
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>();

        match instruction[0] {
            "push" => queue.push_back(instruction[1].to_string()),
            _      => println!(
                "{}",
                match instruction[0] {
                    "size" => queue
                        .len()
                        .to_string(),
                    "empty" => (queue.is_empty() as i32)
                        .to_string(),
                    "pop"   =>
                        if queue.is_empty() { "-1".to_string() }
                        else                { (*queue.pop_front().unwrap()).to_string() },
                    "front" =>
                        if queue.is_empty() { "-1".to_string() }
                        else                { (*queue.front().unwrap()).to_string() },
                    "back"  =>
                        if queue.is_empty() { "-1".to_string() }
                        else                { (*queue.back().unwrap()).to_string() },
                    _       => panic!("unknown instruction"),
                }
            ),
        }
    }
}