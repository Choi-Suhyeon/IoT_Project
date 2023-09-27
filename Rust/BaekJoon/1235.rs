// 1235번 학생 번호

use std::collections::HashSet;
use std::io;

fn get_answer(student_nums: &[String]) -> usize {
    fn _get_answer(student_nums: Vec<Vec<char>>, k: usize) -> usize {
        if k == student_nums[0].len() { return k; }

        let interim = student_nums
            .iter()
            .map(|x| &x[..k])
            .collect::<HashSet<_>>();

        if interim.len() != student_nums.len() { _get_answer(student_nums, k + 1) }
        else                                   { k }
    }

    _get_answer(
        student_nums
            .iter()
            .map(|x| x
                .chars()
                .rev()
                .collect::<Vec<_>>()
            )
            .collect::<_>(),
        1,
    )
}

fn read_from_stdin() -> String {
    let mut raw_input = String::new();

    io::stdin()
        .read_line(&mut raw_input)
        .expect("failed to read");

    raw_input
        .trim()
        .to_string()
}

fn read_usize() -> usize {
    read_from_stdin()
        .parse()
        .expect("expected to input integer")
}

fn main() {
    let num_of_student_nums = read_usize();

    let mut student_nums = Vec::with_capacity(num_of_student_nums);

    (0..num_of_student_nums).for_each(|_| {
        student_nums.push(read_from_stdin());
    });

    println!("{}", get_answer(&student_nums));
}