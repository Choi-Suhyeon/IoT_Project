// 1308번 D-Day

use std::io;

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct Date {
    year:  usize,
    month: usize,
    day:   usize,
}

impl Date {
    fn convert_days(&self) -> usize {
        fn is_leap_year(year: usize) -> bool {
            !(year & 3 != 0 || year & 3 == 0 && year % 100 == 0 && year % 400 != 0)
        }

        let months =
            if is_leap_year(self.year) { [0usize, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31] }
            else                       { [0usize, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31] };

        self.day + months[..self.month].iter().sum::<usize>() + (1..self.year).fold(0usize, |acc, x| {
            acc + if is_leap_year(x) { 366 } else { 365 }
        })
    }
}

fn read_date() -> Date {
    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("failed to read from stdin");

    let [year, month, day] = buffer
        .trim()
        .split_whitespace()
        .map(|s| s
            .parse()
            .expect("invalid input format")
        )
        .collect::<Vec<_>>()[..]
    else {
        panic!("invalid input format");
    };

    Date { year, month, day }
}

fn main() {
    let today    = read_date();
    let criteria = read_date();
    let limit    = Date { year: today.year + 1000, ..today };
    let result   = criteria.convert_days() - today.convert_days();

    if criteria < limit {
        println!("D-{}", result);
    }
    else {
        println!("gg");
    }
}