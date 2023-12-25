//10869 사칙연산 문제

// std::io 모듈을 사용하기 위해 import함
use std::io;

fn main() {
    // 빈 문자열을 생성하여 사용자의 입력을 저장할 변수 생성
    let mut numbersArry = String::new();

    // 사용자의 입력을 읽어들이고 에러 처리
    io::stdin().read_line(&mut numbersArry).unwrap();

    // 입력된 숫자들을 공백으로 나누어 벡터에 저장함
    let numbers: Vec<&str> = numbersArry.split_whitespace().collect();

    // 벡터의 첫 번째와 두 번째 숫자를 정수로 변환
    let number_first: i32 = numbers[0].parse::<i32>().unwrap();
    let number_second: i32 = numbers[1].parse::<i32>().unwrap();

    // 더하기, 빼기, 곱하기, 나누기, 나머지 연산을 수행하고 결과를 출력
    println!("덧셈 결과: {}", number_first + number_second);
    println!("뺄셈 결과: {}", number_first - number_second);
    println!("곱셈 결과: {}", number_first * number_second);

    // 나누기 연산 결과 출력 전에 나누는 수가 0이 아닌지 확인
    if number_second != 0 {
        println!("나눗셈 결과: {}", number_first / number_second);
        println!("나머지 결과: {}", number_first % number_second);
    } else {
        println!("나누는 수가 0이므로 나눗셈 및 나머지 연산 불가능");
    }
}
