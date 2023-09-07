//수 정렬하기

use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("입력을 읽을 수 없습니다.");

    let input: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("유효한 숫자를 입력하세요.");
            return;
        }
    };

    // 크기만큼의 배열 선언
    // 최대 크기를 1000으로 가정
    let mut arr: [i32; 1000] = [1000000000; 1000];

    // 배열 초기화
    for i in 0..input {
        let mut input2 = String::new();

        io::stdin().read_line(&mut input2)
            .expect("입력을 읽을 수 없습니다.");

        let value: i32 = match input2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("유효한 숫자를 입력하세요.");
                -1
            }
        };
        arr[i] = value;
    }
    arr.sort();

    for i in 0..input{
        println!("{}", arr[i]);
    }
}
