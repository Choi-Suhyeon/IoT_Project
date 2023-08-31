//10869번 사칙연산

use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
               .expect("형식에 맞게 입력해주세요.");

    // 문자열을 공백을 기준으로 분리하고 정수로 변환
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|st| st.trim().parse().expect("정수로 변환 실패"))
        .collect();

    let (num1, num2) = match numbers.as_slice() {
        [x, y] => (*x, *y),
        _ => {  //_ = 와일드카드 패턴
           println!("두 개의 정수를 입력하세요.");
            return;
        }
    };

    //println!("입력된 숫자: {} {}", num1, num2);

    let sum = num1 + num2;
    let sub = num1 - num2;
    let mul = num1 * num2;
    let div = num1 / num2;
    let rem = num1 % num2;

    println!("{}", sum);
    println!("{}", sub);
    println!("{}", mul);
    println!("{}", div);
    println!("{}", rem);
}
