// 7568번 덩치

use std::str::FromStr;
use std::io;

// 입력 받는 형식은 이제야 형식화 된 듯: 
// read_from_stdin 하나는 무조건 만들고 입력에 대한 처리가 필요하면 각 처리마다 함수 만들기
// 이렇게 만들어놔도 언제 바뀔 지는 모르겠지만
fn read_from_stdin() -> String {
    let mut raw_input = String::new();

    io::stdin()
        .read_line(&mut raw_input)
        .expect("failed to read");

    raw_input
}

fn read_usize() -> usize {
    read_from_stdin()
        .trim()
        .parse()
        .expect("expected to input integer")
}

fn read_two_i32s() -> (i32, i32) {
    let [x, y] = read_from_stdin()
        .trim()
        .split_whitespace()
        .map(|x|
            i32::from_str(x).expect("expected to input integer")
        )
        .collect::<Vec<_>>()[..]
    else {
        panic!("input form is invalid")
    };

    (x, y)
}

fn main() {
    let num_of_students = read_usize();

    let mut h_w_of_students = Vec::with_capacity(num_of_students);

    (0..num_of_students).for_each(|_| {
        h_w_of_students.push(read_two_i32s());
    });

    (0..num_of_students).for_each(|i| {
        let mut grade = 1;

        (0..num_of_students).for_each(|j| {
            if i == j { return; }

            let criteria       = h_w_of_students[i];
            let to_be_compared = h_w_of_students[j];

            if criteria.0 < to_be_compared.0 && criteria.1 < to_be_compared.1 { grade += 1; }
        });

        print!("{} ", grade);
    });
}
