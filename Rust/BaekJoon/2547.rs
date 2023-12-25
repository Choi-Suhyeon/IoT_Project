// 2547번 : 사탕 선생 고창영

use std::io;

fn read_line() -> String {
    let mut raw_input = String::new();

    io::stdin()
        .read_line(&mut raw_input)
        .expect("failed to read");

    String::from(raw_input.trim())
}

fn read_u128() -> u128 {
    read_line()
        .parse()
        .expect("expected to input integer")
}

fn main() {
    let num_of_cases = read_u128();

    let mut result = String::new();

    (0..num_of_cases).for_each(|_| {
        read_line();

        let num_of_students = read_u128();

        let mut total_candies = 0u128;

        (0..num_of_students).for_each(|_| {
            total_candies += read_u128();
        });

        result.push_str(if total_candies % num_of_students == 0 { "YES\n" } else { "NO\n" });
    });

    print!("{result}");
}