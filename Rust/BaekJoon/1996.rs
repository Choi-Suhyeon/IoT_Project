// 1996번 지뢰 찾기

use std::io;

fn read_from_stdin() -> String {
    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("failed to read");

    buffer
        .trim()
        .to_string()
}

fn read_usize() -> usize {
    read_from_stdin()
        .parse()
        .expect("expected to input integer")
}

fn main() {
    let height = read_usize();
    
    let mut map = Vec::<Vec<isize>>::with_capacity(height);

    for _ in 0..height {
        map.push(read_from_stdin()
            .chars()
            .map(|c| c
                .to_digit(10)
                .or(Some(0))
                .unwrap() as isize
            )
            .collect()
        )
    }

    let mut result = vec![vec![0; map[0].len()]; map.len()];

    for (i, v) in map.iter().enumerate() {
        for (j, &u) in v.iter().enumerate() {
            if u == 0 {
                continue;
            }

            result[i][j] = -1;

            for k in i - (i != 0) as usize ..= i + (i != map.len() - 1) as usize {
                for l in j - (j != 0) as usize ..= j + (j != v.len() - 1) as usize {
                    result[k][l] += if result[k][l] >= 0 { u } else { 0 };
                }
            }
        }
    }

    for i in result {
        println!("{}", i
            .iter()
            .map(|&i| match i {
                n@0..=9 => n.to_string(),
                -1      => "*".to_string(),
                _       => "M".to_string(),
            })
            .fold(String::new(), |acc, s| acc + &s)
        );
    }
}