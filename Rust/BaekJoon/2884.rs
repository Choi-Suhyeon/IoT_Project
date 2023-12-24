// 알람 시계

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Wrong input");

    let mut iter = input.trim().split_whitespace();

    let h: u32 = iter.next().expect("Missing hour").parse().expect("Invalid hour");
    let m: u32 = iter.next().expect("Missing minute").parse().expect("Invalid minute");

    let t = (h * 60) + m;

    let time = if t < 45{
        (24*60) + t - 45
    } else {
        t - 45
    };    

    let hh = time / 60;
    let mm = time % 60;

    println!("{} {}", hh, mm);
}
