// 1018번 체스판 다시 칠하기

use std::io;

fn main() {
    let mut raw_input = String::new();
    let mut board     = Vec::new();

    io::stdin()
        .read_line(&mut raw_input)
        .expect("failed to read");

    let [height, width] = raw_input
        .trim()
        .split_whitespace()
        .map(|x| x
            .parse()
            .expect("expected to input integer")
        )
        .collect::<Vec<usize>>()[..]
    else {
        panic!("input format is invalid");
    };

    (0..height).for_each(|_| {
        let mut raw_input = String::new();

        io::stdin()
            .read_line(&mut raw_input)
            .expect("failed to read");

        board.push(raw_input
            .trim()
            .chars()
            .collect::<Vec<char>>()
        );
    });

    let mut min_fill_incorrect  = 64;
    let mut val_of_is_initial_w = true;

    for _ in 0..2 {
        for h in 0..=height - 8 {
            for w in 0..=width - 8 {
                let mut is_initial_w       = val_of_is_initial_w;
                let mut num_fill_incorrect = 0;

                for hc in h..h + 8 {
                    let mut is_square_w = is_initial_w;

                    for wc in w..w + 8 {
                        if is_square_w && board[hc][wc] != 'W'
                            || !is_square_w && board[hc][wc] != 'B' {
                            num_fill_incorrect += 1;
                        }
                        is_square_w = !is_square_w;
                    }

                    is_initial_w = !is_initial_w;
                }

                if num_fill_incorrect < min_fill_incorrect {
                    min_fill_incorrect = num_fill_incorrect;
                }
            }
        }

        val_of_is_initial_w = !val_of_is_initial_w;
    }

    println!("{min_fill_incorrect}");
}