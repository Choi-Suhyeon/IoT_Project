// 1384번 메시지

use std::fmt::{Debug, Write};
use std::str;
use std::io;

struct Paper {
    name:     String,
    messages: Vec<String>,
}

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

fn get_papers(n: usize) -> Vec<Paper> {
    let mut buffer = String::new();
    let mut result = Vec::new();

    for _ in 0..n {
        io::stdin()
            .read_line(&mut buffer)
            .expect("failed to read");

        let mut raw_input = buffer
            .trim()
            .split_whitespace();

        result.push(Paper {
            name: String::from(raw_input
                .next()
                .expect("invalid input format")
            ),
            messages: raw_input
                .map(String::from)
                .collect::<Vec<String>>(),
        });
        buffer.clear();
    }

    result
}

fn main() {
    let mut to_be_printed = String::new();
    let mut group_num     = 1;

    loop {
        let num = read_one::<usize>();

        if num == 0 {
            break;
        }

        let papers = get_papers(num);

        let mut has_bully = false;

        writeln!(to_be_printed, "Group {}", group_num).unwrap();
        group_num += 1;

        for (i, p) in papers.iter().enumerate() {
            for (j, v) in p.messages.iter().enumerate() {
                if v.as_str() == "N" {
                    let idx_of_bully  = (i as isize - (j as isize + 1)).rem_euclid(num as isize) as usize;
                    let name_of_bully = papers[idx_of_bully].name.clone();

                    has_bully = true;
                    writeln!(to_be_printed, "{} was nasty about {}", name_of_bully, p.name).unwrap();
                }
            }
        }

        if !has_bully {
            to_be_printed.push_str("Nobody was nasty\n");
        }

        to_be_printed.push('\n');
    }

    println!("{}", to_be_printed);
}