//10866번 덱

use std::collections::VecDeque;
use std::io;

fn main() {
    let mut raw_input = String::new();
    let mut deque     = VecDeque::new();

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
            .split(" ")
            .collect::<Vec<&str>>();

        match instruction[0] {
            "push_front" => deque.push_front(instruction[1].clone().to_string()),
            "push_back"  => deque.push_back(instruction[1].clone().to_string()),
            _            => println!(
                "{}",
                match instruction[0] {
                    "size"      => deque.len().to_string(),
                    "empty"     => (deque.is_empty() as i32).to_string(),
                    "front"     =>
                        if deque.is_empty() { "-1".to_string() }
                        else                { (*deque.front().unwrap()).to_string() },
                    "back"      =>
                        if deque.is_empty() { "-1".to_string() }
                        else                { (*deque.back().unwrap()).to_string() },
                    "pop_front" =>
                        if deque.is_empty() { "-1".to_string() }
                        else                { (*deque.pop_front().unwrap()).to_string() },
                    "pop_back"  =>
                        if deque.is_empty() { "-1".to_string() }
                        else                { (*deque.pop_back().unwrap()).to_string() },
                    _           => panic!("unknown instruction"),
                }
            ),
        }
    }
}
