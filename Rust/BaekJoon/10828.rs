//10828번 스택

use std::io;

fn main() {
    let mut raw_input = String::new();
    let mut stack     = Vec::new();

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
            "push"  => stack.push(instruction[1].parse::<i32>().unwrap()),
            "pop"   => println!(
                "{}",
                if stack.is_empty() { -1 }
                else                { stack.pop().unwrap() },
            ),
            "top"   => println!(
                "{}",
                if stack.is_empty() { -1 }
                else                { stack[stack.len() - 1] },
            ),
            "size"  => println!("{}", stack.len()),
            "empty" => println!("{}", stack.is_empty() as i32),
            _       => panic!("unknown instruction"),
        };
    }
}
