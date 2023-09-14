use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("형식에 맞게 입력해주세요.");

    // 문자열을 공백으로 나누기
    let parts: Vec<&str> = input.split_whitespace().collect();

    let (str1, str2) = match parts.as_slice() {
        [x, y] => (*x, *y),
        _ => {                  //_ = 와일드카드 패턴
            println!("두 개의 수를 입력하세요.");
            return;
        }
    };

    // 문자열을 반대로 뒤집기
    let reversed_str1: String = str1.chars().rev().collect();
    let reversed_str2: String = str2.chars().rev().collect();

    // 문자열을 정수로 변환
    match reversed_str1.parse::<i32>() {
        Ok(_str) => { },
        Err(_) => {
            println!("유효한 값이 아닙니다.");
        }
    }
    match reversed_str2.parse::<i32>() {
        Ok(_str) => { },
        Err(_) => {
            println!("유효한 값이 아닙니다.");
        }
    }

    if reversed_str1 > reversed_str2{
        println!("{}", reversed_str1);
    }
    else if reversed_str2 > reversed_str1{
        println!("{}", reversed_str2);
    }

}
