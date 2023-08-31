//1251번 단어 나누기

use std::io;

fn reverse_str(target: &str) -> String {
    target
        .chars()
        .clone()
        .into_iter()
        .rev()
        .collect::<String>()
}

fn main() {
    let mut raw_input = String::new();
    let mut all_conds = Vec::new();

    io::stdin()
        .read_line(&mut raw_input)
        .expect("failed to read");

    let input = raw_input.trim();

    for i in 1..input.len() - 1 {
        for j in i + 1..input.len() {
            let fst_str = reverse_str(&input[..i]);
            let snd_str = reverse_str(&input[i..j]);
            let trd_str = reverse_str(&input[j..]);

            all_conds.push(fst_str + &*snd_str + &*trd_str);
        }
    }

    let result = all_conds
        .iter()
        .min()
        .unwrap();

    println!("{result}");
}