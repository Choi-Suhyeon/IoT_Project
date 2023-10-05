// 4949번 균형잡힌 세상
/*
꼼수 아닌 꼼수를 되도 않게 두 군데나 썼는데, 다른 사람 풀이를 보니 
그런 꼼수 안 써도 생각을 조금만 더 했으면 쉽게 풀 수 있었을 듯
*/

use std::io;

fn read_from_stdin() -> String {
    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("failed to read");

    buffer
        .trim_end()
        .to_string()
}

fn main() {
    let mut result = String::new();

    loop {
        let target = read_from_stdin();

        if &target == "." {
            break;
        }

        let mut stack = Vec::new();

        result.push_str('outer: loop {
            for i in target.chars().collect::<Vec<_>>() {
                match i {
                    '(' | '[' => stack.push(i),
                    ')' | ']' => match (stack.pop(), ' ') {
                        (None, v) | (Some(v), _) if v != if i == ')' { '(' } else { '[' } => {
                            break 'outer "no\n";
                        },
                        _ => {},
                    },
                    _ => {},
                }
            }

            break if stack.is_empty() { "yes\n" } else { "no\n" };
        });
    }

    print!("{result}");
}