// 1439번 뒤집기

use std::cmp::min;
use std::io;

fn partition_whenever<R, T, I>(mut iter: I, f: fn(&R) -> T) -> Option<Vec<Vec<R>>>
where
    R: Copy,
    T: Eq,
    I: Iterator<Item=R>
{
    let first_elem = iter.next()?;

    let mut result = vec![vec![first_elem]];
    let mut pre    = f(&first_elem);

    for i in iter {
        let pred_result = f(&i);

        if pre != pred_result {
            result.push(Vec::new());
        }

        result
            .last_mut()
            .unwrap()
            .push(i);

        pre = pred_result;
    }

    Some(result)
}

fn read_from_stdin() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("failed to read line");

    buffer
        .trim()
        .to_string()
}

fn main() {
    let input = read_from_stdin();
    let input = input.chars();

    let result = partition_whenever::<_, _, _>(input, |&c| c)
        .unwrap()
        .iter()
        .map(|x| *x.first().unwrap())
        .partition::<Vec<char>, _>(|&x| x == '0');

    println!("{}", min(result.0.len(), result.1.len()));
}