//단어 길이 재기

use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("입력을 읽을 수 없습니다.");

    // 공백 및 개행 문자를 제거하여 알파벳만 남김
    let word = input.trim().chars().filter(|s| s.is_alphabetic()).collect::<String>();
    let length = word.len();

    println!("{}", length);
}
